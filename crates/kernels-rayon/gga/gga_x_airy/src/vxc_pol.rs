//! GGA_X_AIRY vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
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
pub fn gga_x_airy_vxc_pol(
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
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t29 * t32;
            let t34 = ((v_sigma0).sqrt());
            let t35 = (simd::cbrt(v_rho0));
            let t37 = f64x8::splat(1.0) / t35 / v_rho0;
            let t39 = t33 * t34 * t37;
            let t40 = (simd::pow(t39, f64x8::splat(2.626712)));
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t40;
            let t43 = (simd::pow(t42, -f64x8::splat(0.657946)));
            let t46 = (simd::pow(t39, f64x8::splat(3.217063)));
            let t48 = (simd::pow(t39, f64x8::splat(3.223476)));
            let t50 = f64x8::splat(1.0) - f64x8::splat(0.04521241301076986) * t46 + f64x8::splat(0.04540222195662038) * t48;
            let t51 = (simd::pow(t39, f64x8::splat(3.473804)));
            let t53 = f64x8::splat(1.0) + f64x8::splat(0.0004770218022490335) * t51;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = f64x8::splat(6.014601922021111e-05) * t40 * t43 + t50 * t54;
            let t60 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t56));
            let t61 = (v_rho1).simd_le(dens_threshold);
            let t62 = -t16;
            let t64 = ((t14).select(t11, (t10).select(t15, t62 * t7)));
            let t65 = f64x8::splat(1.0) + t64;
            let t66 = (t65).simd_le(zeta_threshold);
            let t67 = (simd::cbrt(t65));
            let t69 = ((t66).select(t22, t67 * t65));
            let t70 = t69 * t26;
            let t71 = ((v_sigma2).sqrt());
            let t72 = (simd::cbrt(v_rho1));
            let t74 = f64x8::splat(1.0) / t72 / v_rho1;
            let t76 = t33 * t71 * t74;
            let t77 = (simd::pow(t76, f64x8::splat(2.626712)));
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t77;
            let t80 = (simd::pow(t79, -f64x8::splat(0.657946)));
            let t83 = (simd::pow(t76, f64x8::splat(3.217063)));
            let t85 = (simd::pow(t76, f64x8::splat(3.223476)));
            let t87 = f64x8::splat(1.0) - f64x8::splat(0.04521241301076986) * t83 + f64x8::splat(0.04540222195662038) * t85;
            let t88 = (simd::pow(t76, f64x8::splat(3.473804)));
            let t90 = f64x8::splat(1.0) + f64x8::splat(0.0004770218022490335) * t88;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = f64x8::splat(6.014601922021111e-05) * t77 * t80 + t87 * t91;
            let t97 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t70 * t93));
            let tzk0 = t60 + t97;
            acc_zk = tzk0;
            let t98 = t6 * t6;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t16 * t99;
            let t102 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t100)));
            let t105 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t102));
            let t106 = t105 * t26;
            let t110 = t26 * t26;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t25 * t111;
            let t115 = t5 * t112 * t56 / f64x8::splat(8.0);
            let t116 = (simd::pow(t39, f64x8::splat(1.626712)));
            let t118 = t116 * t43 * t29;
            let t119 = t32 * t34;
            let t120 = v_rho0 * v_rho0;
            let t122 = f64x8::splat(1.0) / t35 / t120;
            let t123 = t119 * t122;
            let t126 = (simd::pow(t39, f64x8::splat(4.253424)));
            let t127 = (simd::pow(t42, -f64x8::splat(1.657946)));
            let t129 = t126 * t127 * t29;
            let t132 = (simd::pow(t39, f64x8::splat(2.217063)));
            let t133 = t132 * t29;
            let t136 = (simd::pow(t39, f64x8::splat(2.223476)));
            let t137 = t136 * t29;
            let t140 = f64x8::splat(0.19393490805022173) * t133 * t123 - f64x8::splat(0.19513729709845176) * t137 * t123;
            let t142 = t53 * t53;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t50 * t143;
            let t145 = (simd::pow(t39, f64x8::splat(2.473804)));
            let t146 = t144 * t145;
            let t148 = t33 * t34 * t122;
            let t151 = -f64x8::splat(0.00021064836058394556) * t118 * t123 + f64x8::splat(1.8671024483029836e-08) * t129 * t123 + t140 * t54 + f64x8::splat(0.0022094403263198687) * t146 * t148;
            let t156 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t106 * t56 - t115 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t151));
            let t157 = t62 * t99;
            let t159 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t157)));
            let t162 = ((t66).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t67 * t159));
            let t163 = t162 * t26;
            let t167 = t69 * t111;
            let t170 = t5 * t167 * t93 / f64x8::splat(8.0);
            let t172 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t163 * t93 - t170));
            let tvrho0 = t60 + t97 + t6 * (t156 + t172);
            acc_vrho_0 = tvrho0;
            let t176 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t100)));
            let t179 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t176));
            let t180 = t179 * t26;
            let t185 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t56 - t115));
            let t187 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t157)));
            let t190 = ((t66).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t67 * t187));
            let t191 = t190 * t26;
            let t195 = (simd::pow(t76, f64x8::splat(1.626712)));
            let t197 = t195 * t80 * t29;
            let t198 = t32 * t71;
            let t199 = v_rho1 * v_rho1;
            let t201 = f64x8::splat(1.0) / t72 / t199;
            let t202 = t198 * t201;
            let t205 = (simd::pow(t76, f64x8::splat(4.253424)));
            let t206 = (simd::pow(t79, -f64x8::splat(1.657946)));
            let t208 = t205 * t206 * t29;
            let t211 = (simd::pow(t76, f64x8::splat(2.217063)));
            let t212 = t211 * t29;
            let t215 = (simd::pow(t76, f64x8::splat(2.223476)));
            let t216 = t215 * t29;
            let t219 = f64x8::splat(0.19393490805022173) * t212 * t202 - f64x8::splat(0.19513729709845176) * t216 * t202;
            let t221 = t90 * t90;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t87 * t222;
            let t224 = (simd::pow(t76, f64x8::splat(2.473804)));
            let t225 = t223 * t224;
            let t227 = t33 * t71 * t201;
            let t230 = -f64x8::splat(0.00021064836058394556) * t197 * t202 + f64x8::splat(1.8671024483029836e-08) * t208 * t202 + t219 * t91 + f64x8::splat(0.0022094403263198687) * t225 * t227;
            let t235 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t191 * t93 - t170 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t70 * t230));
            let tvrho1 = t60 + t97 + t6 * (t185 + t235);
            acc_vrho_1 = tvrho1;
            let t238 = f64x8::splat(1.0) / t34;
            let t239 = t32 * t238;
            let t240 = t239 * t37;
            let t249 = -f64x8::splat(0.07272559051883315) * t133 * t240 + f64x8::splat(0.07317648641191941) * t137 * t240;
            let t252 = t33 * t238 * t37;
            let t255 = f64x8::splat(7.899313521897959e-05) * t118 * t240 - f64x8::splat(7.001634181136188e-09) * t129 * t240 + t249 * t54 - f64x8::splat(0.0008285401223699508) * t146 * t252;
            let t259 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t255));
            let tvsigma0 = t6 * t259;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t260 = f64x8::splat(1.0) / t71;
            let t261 = t32 * t260;
            let t262 = t261 * t74;
            let t271 = -f64x8::splat(0.07272559051883315) * t212 * t262 + f64x8::splat(0.07317648641191941) * t216 * t262;
            let t274 = t33 * t260 * t74;
            let t277 = f64x8::splat(7.899313521897959e-05) * t197 * t262 - f64x8::splat(7.001634181136188e-09) * t208 * t262 + t271 * t91 - f64x8::splat(0.0008285401223699508) * t225 * t274;
            let t281 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t70 * t277));
            let tvsigma2 = t6 * t281;
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
