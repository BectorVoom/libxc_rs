//! MGGA_C_B88 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b88_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t7 = (simd::cbrt(t6));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t5 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t9 * t10;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = ((t13).select(t14, (t13).select(-t14, f64x8::splat(0.0))));
            let t17 = f64x8::splat(1.0) + t16;
            let t18 = t17 * v_rho;
            let t19 = (simd::cbrt(t18));
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t12 * t20;
            let t22 = t12 * t12;
            let t23 = v_sigma * t22;
            let t24 = v_rho * v_rho;
            let t25 = (simd::cbrt(v_rho));
            let t26 = t25 * t25;
            let t28 = f64x8::splat(1.0) / t26 / t24;
            let t29 = t23 * t28;
            let t31 = f64x8::splat(1.0) + f64x8::splat(0.007) * t29;
            let t32 = (simd::pow(t31, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t33 = t32 * t32;
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.0008333333333333334) * t11 * t23 * t28 * t35;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t11 * t21 * t41;
            let t45 = ((t3).select(f64x8::splat(0.0), t43 / f64x8::splat(9.0)));
            let t46 = v_rho * t45;
            let t47 = f64x8::splat(1.26) * t45;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (simd::ln(t48));
            let t50 = t47 - t49;
            let t52 = f64x8::splat(0.252) * t46 * t50;
            let t53 = t17 * t17;
            let t54 = (simd::cbrt(t17));
            let t55 = t54 * t54;
            let t56 = t55 * t53;
            let t57 = t56 * t22;
            let t58 = t26 * v_rho;
            let t59 = v_tau * t22;
            let t64 = f64x8::splat(2.0) * t59 / t58 - t29 / f64x8::splat(4.0);
            let t66 = t58 * t64 * t5;
            let t67 = t57 * t66;
            let t69 = f64x8::splat(1.0) / t7 / t6;
            let t70 = t69 * t10;
            let t72 = f64x8::splat(1.0) / t19 / t18;
            let t73 = t40 * t40;
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t72 * t75;
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.10666666666666667) * t43;
            let t79 = (simd::ln(t78));
            let t80 = t79 * t4;
            let t81 = t80 * t7;
            let t82 = t10 * t10;
            let t83 = t82 * t22;
            let t84 = t19 * t40;
            let t85 = t83 * t84;
            let t88 = f64x8::splat(1.0) - f64x8::splat(0.390625) * t81 * t85;
            let t90 = t70 * t76 * t88;
            let t93 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.0001864135111111111) * t67 * t90));
            let t94 = f64x8::splat(2.0) * t93;
            let tzk0 = -t52 + t94;
            acc_zk = tzk0;
            let t95 = t45 * t50;
            let t97 = t12 * t72;
            let t100 = t11 * t97 * t41 * t17;
            let t102 = f64x8::splat(1.0) / t73;
            let t103 = t24 * v_rho;
            let t105 = f64x8::splat(1.0) / t26 / t103;
            let t110 = v_sigma * v_sigma;
            let t111 = t110 * t12;
            let t112 = t24 * t24;
            let t113 = t112 * t24;
            let t115 = f64x8::splat(1.0) / t25 / t113;
            let t117 = f64x8::splat(1.0) / t34 / t31;
            let t122 = -f64x8::splat(0.0022222222222222222) * t11 * t23 * t105 * t35 + f64x8::splat(2.488888888888889e-05) * t11 * t111 * t115 * t117;
            let t125 = t11 * t21 * t102 * t122;
            let t128 = ((t3).select(f64x8::splat(0.0), -t100 / f64x8::splat(27.0) - t125 / f64x8::splat(9.0)));
            let t129 = v_rho * t128;
            let t130 = t129 * t50;
            let t133 = f64x8::splat(1.0) / t48;
            let t136 = f64x8::splat(1.26) * t128 - f64x8::splat(1.26) * t128 * t133;
            let t137 = t46 * t136;
            let t140 = t26 * t64 * t5;
            let t141 = t57 * t140;
            let t148 = -f64x8::splat(10.0) / f64x8::splat(3.0) * t59 * t28 + f64x8::splat(2.0) / f64x8::splat(3.0) * t23 * t105;
            let t150 = t58 * t148 * t5;
            let t151 = t57 * t150;
            let t154 = t53 * t17;
            let t155 = t55 * t154;
            let t156 = t155 * t22;
            let t157 = t156 * t66;
            let t158 = t53 * t24;
            let t160 = f64x8::splat(1.0) / t19 / t158;
            let t161 = t160 * t75;
            let t163 = t70 * t161 * t88;
            let t166 = t70 * t72;
            let t168 = f64x8::splat(1.0) / t74 / t40;
            let t169 = t168 * t88;
            let t170 = t169 * t122;
            let t171 = t166 * t170;
            let t176 = -f64x8::splat(0.035555555555555556) * t100 - f64x8::splat(0.10666666666666667) * t125;
            let t177 = f64x8::splat(1.0) / t78;
            let t179 = t4 * t7;
            let t180 = t176 * t177 * t179;
            let t184 = t80 * t7 * t82;
            let t185 = t19 * t19;
            let t186 = f64x8::splat(1.0) / t185;
            let t187 = t22 * t186;
            let t188 = t40 * t17;
            let t192 = t19 * t122;
            let t193 = t83 * t192;
            let t196 = -f64x8::splat(0.390625) * t180 * t85 - f64x8::splat(0.13020833333333334) * t184 * t187 * t188 - f64x8::splat(0.390625) * t81 * t193;
            let t198 = t70 * t76 * t196;
            let t202 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.00031068918518518517) * t141 * t90 - f64x8::splat(0.0001864135111111111) * t151 * t90 + f64x8::splat(0.0002485513481481481) * t157 * t163 + f64x8::splat(0.0007456540444444444) * t67 * t171 - f64x8::splat(0.0001864135111111111) * t67 * t198));
            let tvrho0 = -t52 + t94 + v_rho * (-f64x8::splat(0.252) * t95 - f64x8::splat(0.252) * t130 - f64x8::splat(0.252) * t137 + f64x8::splat(2.0) * t202);
            acc_vrho = tvrho0;
            let t211 = t112 * v_rho;
            let t213 = f64x8::splat(1.0) / t25 / t211;
            let t218 = f64x8::splat(0.0008333333333333334) * t11 * t22 * t28 * t35 - f64x8::splat(9.333333333333333e-06) * t11 * v_sigma * t12 * t213 * t117;
            let t219 = t102 * t218;
            let t223 = ((t3).select(f64x8::splat(0.0), -t11 * t21 * t219 / f64x8::splat(9.0)));
            let t224 = v_rho * t223;
            let t226 = f64x8::splat(0.252) * t224 * t50;
            let t230 = f64x8::splat(1.26) * t223 - f64x8::splat(1.26) * t223 * t133;
            let t232 = f64x8::splat(0.252) * t46 * t230;
            let t233 = t56 * t12;
            let t234 = f64x8::splat(1.0) / v_rho;
            let t235 = t234 * t5;
            let t236 = t233 * t235;
            let t239 = t169 * t218;
            let t240 = t166 * t239;
            let t243 = t41 * t218;
            let t246 = t19 * t218;
            let t247 = t83 * t246;
            let t250 = f64x8::splat(1.0) * t243 * t177 - f64x8::splat(0.390625) * t81 * t247;
            let t252 = t70 * t76 * t250;
            let t256 = ((t3).select(f64x8::splat(0.0), f64x8::splat(9.320675555555555e-05) * t236 * t90 + f64x8::splat(0.0007456540444444444) * t67 * t240 - f64x8::splat(0.0001864135111111111) * t67 * t252));
            let t257 = f64x8::splat(2.0) * t256;
            let tvsigma0 = v_rho * (-t226 - t232 + t257);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t259 = t5 * t69;
            let t260 = t233 * t259;
            let t261 = t10 * t72;
            let t262 = t75 * t88;
            let t266 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.0007456540444444444) * t260 * t261 * t262));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t266;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
