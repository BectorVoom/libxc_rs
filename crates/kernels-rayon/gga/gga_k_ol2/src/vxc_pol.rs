//! GGA_K_OL2 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol2.c`
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
pub fn gga_k_ol2_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_bb = f64x8::splat(param_bb);
    let param_cc = f64x8::splat(param_cc);
    let param_aa = f64x8::splat(param_aa);
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = param_bb * v_sigma0;
            let t33 = v_rho0 * v_rho0;
            let t34 = (simd::cbrt(v_rho0));
            let t35 = t34 * t34;
            let t37 = f64x8::splat(1.0) / t35 / t33;
            let t40 = ((v_sigma0).sqrt());
            let t41 = param_cc * t40;
            let t43 = f64x8::splat(1.0) / t34 / v_rho0;
            let t44 = f64x8::splat(M_CBRT2);
            let t47 = f64x8::splat(4.0) * t40 * t43 + t44;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t43 * t48;
            let t51 = param_aa + f64x8::splat(0.013888888888888888) * t32 * t37 + t41 * t49;
            let t55 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t51));
            let t56 = (v_rho1).simd_le(dens_threshold);
            let t57 = -t17;
            let t59 = ((t15).select(t12, (t11).select(t16, t57 * t8)));
            let t60 = f64x8::splat(1.0) + t59;
            let t61 = (t60).simd_le(zeta_threshold);
            let t62 = (simd::cbrt(t60));
            let t63 = t62 * t62;
            let t65 = ((t61).select(t24, t63 * t60));
            let t66 = t65 * t30;
            let t67 = param_bb * v_sigma2;
            let t68 = v_rho1 * v_rho1;
            let t69 = (simd::cbrt(v_rho1));
            let t70 = t69 * t69;
            let t72 = f64x8::splat(1.0) / t70 / t68;
            let t75 = ((v_sigma2).sqrt());
            let t76 = param_cc * t75;
            let t78 = f64x8::splat(1.0) / t69 / v_rho1;
            let t81 = f64x8::splat(4.0) * t75 * t78 + t44;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t78 * t82;
            let t85 = param_aa + f64x8::splat(0.013888888888888888) * t67 * t72 + t76 * t83;
            let t89 = ((t56).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t66 * t85));
            let tzk0 = t55 + t89;
            acc_zk = tzk0;
            let t90 = t7 * t7;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t17 * t91;
            let t94 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t92)));
            let t97 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t94));
            let t98 = t97 * t30;
            let t102 = f64x8::splat(1.0) / t29;
            let t103 = t28 * t102;
            let t106 = t6 * t103 * t51 / f64x8::splat(10.0);
            let t107 = t33 * v_rho0;
            let t109 = f64x8::splat(1.0) / t35 / t107;
            let t114 = f64x8::splat(1.0) / t34 / t33 * t48;
            let t117 = param_cc * v_sigma0;
            let t118 = t47 * t47;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t109 * t119;
            let t123 = -f64x8::splat(0.037037037037037035) * t32 * t109 - f64x8::splat(4.0) / f64x8::splat(3.0) * t41 * t114 + f64x8::splat(16.0) / f64x8::splat(3.0) * t117 * t120;
            let t128 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t98 * t51 + t106 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t123));
            let t129 = t57 * t91;
            let t131 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t129)));
            let t134 = ((t61).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t63 * t131));
            let t135 = t134 * t30;
            let t139 = t65 * t102;
            let t142 = t6 * t139 * t85 / f64x8::splat(10.0);
            let t144 = ((t56).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t135 * t85 + t142));
            let tvrho0 = t55 + t89 + t7 * (t128 + t144);
            acc_vrho_0 = tvrho0;
            let t148 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t92)));
            let t151 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t148));
            let t152 = t151 * t30;
            let t157 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t152 * t51 + t106));
            let t159 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t129)));
            let t162 = ((t61).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t63 * t159));
            let t163 = t162 * t30;
            let t167 = t68 * v_rho1;
            let t169 = f64x8::splat(1.0) / t70 / t167;
            let t174 = f64x8::splat(1.0) / t69 / t68 * t82;
            let t177 = param_cc * v_sigma2;
            let t178 = t81 * t81;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t169 * t179;
            let t183 = -f64x8::splat(0.037037037037037035) * t67 * t169 - f64x8::splat(4.0) / f64x8::splat(3.0) * t76 * t174 + f64x8::splat(16.0) / f64x8::splat(3.0) * t177 * t180;
            let t188 = ((t56).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t85 + t142 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t66 * t183));
            let tvrho1 = t55 + t89 + t7 * (t157 + t188);
            acc_vrho_1 = tvrho1;
            let t193 = f64x8::splat(1.0) / t40;
            let t194 = param_cc * t193;
            let t200 = f64x8::splat(0.013888888888888888) * param_bb * t37 + t194 * t49 / f64x8::splat(2.0) - f64x8::splat(2.0) * param_cc * t37 * t119;
            let t204 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t200));
            let tvsigma0 = t7 * t204;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t207 = f64x8::splat(1.0) / t75;
            let t208 = param_cc * t207;
            let t214 = f64x8::splat(0.013888888888888888) * param_bb * t72 + t208 * t83 / f64x8::splat(2.0) - f64x8::splat(2.0) * param_cc * t72 * t179;
            let t218 = ((t56).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t66 * t214));
            let tvsigma2 = t7 * t218;
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
