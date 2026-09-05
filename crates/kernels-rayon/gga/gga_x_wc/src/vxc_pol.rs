//! GGA_X_WC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_wc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t43 = (simd::exp(-t40 / f64x8::splat(24.0)));
            let t47 = t28 * t28;
            let t49 = f64x8::splat(1.0) / t30 / t29;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t34 * t34;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t35 / t53;
            let t59 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t50 * t51 * t55;
            let t60 = (simd::ln(t59));
            let t61 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t40 + f64x8::splat(0.004002424276710846) * t33 * t39 * t43 + t60;
            let t64 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t61;
            let t68 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t25 * t26 * t64));
            let t69 = (v_rho1).simd_le(dens_threshold);
            let t70 = -t16;
            let t72 = ((t14).select(t11, (t10).select(t15, t70 * t7)));
            let t73 = f64x8::splat(1.0) + t72;
            let t74 = (t73).simd_le(zeta_threshold);
            let t75 = (simd::cbrt(t73));
            let t77 = ((t74).select(t22, t75 * t73));
            let t79 = v_rho1 * v_rho1;
            let t80 = (simd::cbrt(v_rho1));
            let t81 = t80 * t80;
            let t83 = f64x8::splat(1.0) / t81 / t79;
            let t84 = v_sigma2 * t83;
            let t85 = t33 * t84;
            let t88 = (simd::exp(-t85 / f64x8::splat(24.0)));
            let t92 = v_sigma2 * v_sigma2;
            let t93 = t79 * t79;
            let t94 = t93 * v_rho1;
            let t96 = f64x8::splat(1.0) / t80 / t94;
            let t100 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t50 * t92 * t96;
            let t101 = (simd::ln(t100));
            let t102 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t85 + f64x8::splat(0.004002424276710846) * t33 * t84 * t88 + t101;
            let t105 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t102;
            let t109 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t77 * t26 * t105));
            let tzk0 = t68 + t109;
            acc_zk = tzk0;
            let t110 = t6 * t6;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t16 * t111;
            let t114 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t112)));
            let t117 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t114));
            let t122 = t26 * t26;
            let t123 = f64x8::splat(1.0) / t122;
            let t127 = t5 * t25 * t123 * t64 / f64x8::splat(8.0);
            let t128 = t2 * t25;
            let t129 = t61 * t61;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t26 * t130;
            let t132 = t34 * v_rho0;
            let t134 = f64x8::splat(1.0) / t36 / t132;
            let t135 = v_sigma0 * t134;
            let t141 = t52 * t34;
            let t143 = f64x8::splat(1.0) / t35 / t141;
            let t144 = t51 * t143;
            let t148 = f64x8::splat(1.0) / t59;
            let t152 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t135 - f64x8::splat(0.010673131404562256) * t33 * t135 * t43 + f64x8::splat(0.00044471380852342736) * t50 * t144 * t43 - f64x8::splat(7.349508643668351e-05) * t50 * t144 * t148;
            let t153 = t131 * t152;
            let t157 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t117 * t26 * t64 - t127 - f64x8::splat(0.1655109536374632) * t128 * t153));
            let t158 = t70 * t111;
            let t160 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t158)));
            let t163 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t160));
            let t171 = t5 * t77 * t123 * t105 / f64x8::splat(8.0);
            let t173 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t163 * t26 * t105 - t171));
            let tvrho0 = t68 + t109 + t6 * (t157 + t173);
            acc_vrho_0 = tvrho0;
            let t177 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t112)));
            let t180 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t177));
            let t186 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t26 * t64 - t127));
            let t188 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t158)));
            let t191 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t188));
            let t196 = t2 * t77;
            let t197 = t102 * t102;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t26 * t198;
            let t200 = t79 * v_rho1;
            let t202 = f64x8::splat(1.0) / t81 / t200;
            let t203 = v_sigma2 * t202;
            let t209 = t93 * t79;
            let t211 = f64x8::splat(1.0) / t80 / t209;
            let t212 = t92 * t211;
            let t216 = f64x8::splat(1.0) / t100;
            let t220 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t203 - f64x8::splat(0.010673131404562256) * t33 * t203 * t88 + f64x8::splat(0.00044471380852342736) * t50 * t212 * t88 - f64x8::splat(7.349508643668351e-05) * t50 * t212 * t216;
            let t221 = t199 * t220;
            let t225 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t191 * t26 * t105 - t171 - f64x8::splat(0.1655109536374632) * t196 * t221));
            let tvrho1 = t68 + t109 + t6 * (t186 + t225);
            acc_vrho_1 = tvrho1;
            let t233 = v_sigma0 * t55;
            let t240 = f64x8::splat(5.0) / f64x8::splat(972.0) * t33 * t38 + f64x8::splat(0.004002424276710846) * t33 * t38 * t43 - f64x8::splat(0.00016676767819628525) * t50 * t233 * t43 + f64x8::splat(2.7560657413756314e-05) * t50 * t233 * t148;
            let t241 = t131 * t240;
            let t244 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t128 * t241));
            let tvsigma0 = t6 * t244;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t250 = v_sigma2 * t96;
            let t257 = f64x8::splat(5.0) / f64x8::splat(972.0) * t33 * t83 + f64x8::splat(0.004002424276710846) * t33 * t83 * t88 - f64x8::splat(0.00016676767819628525) * t50 * t250 * t88 + f64x8::splat(2.7560657413756314e-05) * t50 * t250 * t216;
            let t258 = t199 * t257;
            let t261 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t196 * t258));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
