//! MGGA_X_REGTM vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
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
pub fn mgga_x_regtm_vxc_unpol(
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
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_tau * t22;
            let t24 = t19 * t19;
            let t26 = f64x8::splat(1.0) / t24 / v_rho;
            let t27 = t23 * t26;
            let t28 = v_sigma * t22;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t24 / t29;
            let t32 = t28 * t31;
            let t34 = t27 - t32 / f64x8::splat(8.0);
            let t35 = f64x8::splat(M_CBRT6);
            let t36 = t34 * t35;
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t35 * t40;
            let t42 = t41 * t32;
            let t44 = t36 * t40;
            let t46 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t44;
            let t47 = t46 * t46;
            let t48 = t47 * t46;
            let t49 = t34 * t34;
            let t50 = t35 * t35;
            let t51 = t49 * t50;
            let t53 = f64x8::splat(1.0) / t38 / t37;
            let t56 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t51 * t53;
            let t57 = ((t56).sqrt());
            let t59 = f64x8::splat(1.0) / t57 / t56;
            let t60 = t48 * t59;
            let t62 = (simd::exp(-t42 / f64x8::splat(8.0)));
            let t64 = t42 / f64x8::splat(24.0) + t60 * t62;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t40 * t65;
            let t69 = f64x8::splat(1.0) + t36 * t66 / f64x8::splat(3.0);
            let t70 = t69 * t69;
            let t72 = t70 * t69;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = f64x8::splat(1.0) / t70 + f64x8::splat(3.0) * t73;
            let t76 = f64x8::splat(1.0) + t73;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t75 * t78;
            let t81 = t50 * t53;
            let t82 = v_sigma * v_sigma;
            let t83 = t82 * t21;
            let t84 = t29 * t29;
            let t85 = t84 * v_rho;
            let t87 = f64x8::splat(1.0) / t19 / t85;
            let t91 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t42 + f64x8::splat(0.00537989809245259) * t81 * t83 * t87;
            let t92 = (simd::pow(t91, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t103 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t42 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t27 + f64x8::splat(0.256337604) * t50 * t39 + f64x8::splat(0.011867481666666667) * t32) * t35 * t40;
            let t104 = t92 * t92;
            let t105 = f64x8::splat(1.0) / t104;
            let t108 = f64x8::splat(1.0) / t92 + f64x8::splat(7.0) / f64x8::splat(9.0) * t103 * t105;
            let t110 = f64x8::splat(1.0) - t79;
            let t113 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t42) * t35;
            let t114 = t113 * t40;
            let t119 = t44 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t42 / f64x8::splat(36.0);
            let t120 = t119 * t119;
            let t122 = f64x8::splat(1.0) / v_rho;
            let t123 = v_sigma * t122;
            let t124 = f64x8::splat(1.0) / v_tau;
            let t126 = t123 * t124 / f64x8::splat(8.0);
            let t127 = (t126).simd_lt(f64x8::splat(1.0));
            let t128 = ((t127).select(t126, f64x8::splat(1.0)));
            let t129 = t119 * t128;
            let t130 = f64x8::splat(1.0) - t128;
            let t133 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t114 * t32 + f64x8::splat(292.0) / f64x8::splat(405.0) * t120 - f64x8::splat(146.0) / f64x8::splat(135.0) * t129 * t130;
            let t134 = (simd::pow(t133, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t136 = t108 * t79 + t110 * t134;
            let t140 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t136));
            let tzk0 = f64x8::splat(2.0) * t140;
            acc_zk = tzk0;
            let t142 = t18 / t24;
            let t146 = t23 * t31;
            let t148 = t29 * v_rho;
            let t150 = f64x8::splat(1.0) / t24 / t148;
            let t151 = t28 * t150;
            let t153 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t146 + t151 / f64x8::splat(3.0);
            let t154 = t153 * t35;
            let t156 = t64 * t64;
            let t158 = t40 / t156;
            let t159 = t41 * t151;
            let t161 = t47 * t59;
            let t162 = t161 * t62;
            let t163 = t154 * t40;
            let t166 = t56 * t56;
            let t168 = f64x8::splat(1.0) / t57 / t166;
            let t169 = t48 * t168;
            let t170 = t169 * t62;
            let t171 = t34 * t50;
            let t172 = t53 * t153;
            let t173 = t171 * t172;
            let t176 = t60 * t41;
            let t177 = t150 * t62;
            let t181 = -t159 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t163 - f64x8::splat(2.0144675925925926) * t170 * t173 + t176 * t28 * t177 / f64x8::splat(3.0);
            let t182 = t158 * t181;
            let t185 = t154 * t66 / f64x8::splat(3.0) - t36 * t182 / f64x8::splat(3.0);
            let t188 = t70 * t70;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t189 * t185;
            let t192 = -f64x8::splat(2.0) * t185 * t73 - f64x8::splat(9.0) * t190;
            let t193 = t192 * t78;
            let t196 = f64x8::splat(1.0) / t77 / t76;
            let t197 = t75 * t196;
            let t198 = t108 * t189;
            let t199 = t198 * t185;
            let t203 = f64x8::splat(1.0) / t92 / t91;
            let t205 = t84 * t29;
            let t207 = f64x8::splat(1.0) / t19 / t205;
            let t209 = t81 * t83 * t207;
            let t211 = -f64x8::splat(0.40121303703703703) * t159 - f64x8::splat(0.028692789826413812) * t209;
            let t221 = -f64x8::splat(0.17051554074074074) * t159 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t146 - f64x8::splat(0.031646617777777775) * t151) * t35 * t40;
            let t225 = f64x8::splat(1.0) / t104 / t91;
            let t226 = t103 * t225;
            let t229 = -t203 * t211 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t221 * t105 - f64x8::splat(14.0) / f64x8::splat(45.0) * t226 * t211;
            let t233 = -f64x8::splat(6.0) * t190 * t197 - t193;
            let t235 = t134 * t134;
            let t236 = t235 * t235;
            let t237 = t236 * t236;
            let t238 = t237 * t134;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t110 * t239;
            let t246 = t163 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t159;
            let t249 = t246 * t128;
            let t252 = f64x8::splat(1.0) / t29;
            let t253 = v_sigma * t252;
            let t256 = ((t127).select(-t253 * t124 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t257 = t119 * t256;
            let t262 = -f64x8::splat(125.0) / f64x8::splat(19683.0) * t209 - f64x8::splat(10.0) / f64x8::splat(9.0) * t114 * t151 + f64x8::splat(584.0) / f64x8::splat(405.0) * t119 * t246 - f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t130 - f64x8::splat(146.0) / f64x8::splat(135.0) * t257 * t130 + f64x8::splat(146.0) / f64x8::splat(135.0) * t129 * t256;
            let t265 = t193 * t108 + f64x8::splat(6.0) * t197 * t199 + t79 * t229 + t233 * t134 + t240 * t262 / f64x8::splat(10.0);
            let t270 = ((t3).select(f64x8::splat(0.0), -t7 * t142 * t136 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t265));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t270 + f64x8::splat(2.0) * t140;
            acc_vrho = tvrho0;
            let t273 = t22 * t31;
            let t274 = t41 * t65;
            let t275 = t273 * t274;
            let t277 = t273 * t41;
            let t279 = t162 * t277;
            let t281 = t62 * t34;
            let t282 = t169 * t281;
            let t283 = t81 * t273;
            let t284 = t282 * t283;
            let t286 = t60 * t22;
            let t287 = t31 * t35;
            let t288 = t40 * t62;
            let t292 = t277 / f64x8::splat(24.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t279 + f64x8::splat(0.25180844907407407) * t284 - t286 * t287 * t288 / f64x8::splat(8.0);
            let t293 = t158 * t292;
            let t296 = -t275 / f64x8::splat(24.0) - t36 * t293 / f64x8::splat(3.0);
            let t299 = t189 * t296;
            let t301 = -f64x8::splat(2.0) * t296 * t73 - f64x8::splat(9.0) * t299;
            let t302 = t301 * t78;
            let t304 = t198 * t296;
            let t308 = v_sigma * t21;
            let t309 = t308 * t87;
            let t310 = t81 * t309;
            let t312 = f64x8::splat(0.1504548888888889) * t277 + f64x8::splat(0.01075979618490518) * t310;
            let t315 = t41 * t105;
            let t320 = -t203 * t312 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t273 * t315 - f64x8::splat(14.0) / f64x8::splat(45.0) * t226 * t312;
            let t324 = -f64x8::splat(6.0) * t197 * t299 - t302;
            let t327 = t40 * t22;
            let t331 = t119 * t22;
            let t332 = t287 * t40;
            let t333 = t331 * t332;
            let t335 = t273 * t35;
            let t336 = t40 * t128;
            let t337 = t336 * t130;
            let t338 = t335 * t337;
            let t342 = ((t127).select(t122 * t124 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t343 = t119 * t342;
            let t348 = f64x8::splat(125.0) / f64x8::splat(52488.0) * t310 + f64x8::splat(5.0) / f64x8::splat(12.0) * t113 * t327 * t31 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t333 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t338 - f64x8::splat(146.0) / f64x8::splat(135.0) * t343 * t130 + f64x8::splat(146.0) / f64x8::splat(135.0) * t129 * t342;
            let t351 = t302 * t108 + f64x8::splat(6.0) * t197 * t304 + t79 * t320 + t324 * t134 + t240 * t348 / f64x8::splat(10.0);
            let t355 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t351));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t355;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t357 = t22 * t26;
            let t359 = t357 * t41;
            let t362 = t81 * t357;
            let t365 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t359 - f64x8::splat(2.0144675925925926) * t282 * t362;
            let t366 = t158 * t365;
            let t369 = t357 * t274 / f64x8::splat(3.0) - t36 * t366 / f64x8::splat(3.0);
            let t372 = t189 * t369;
            let t374 = -f64x8::splat(2.0) * t369 * t73 - f64x8::splat(9.0) * t372;
            let t375 = t374 * t78;
            let t377 = t198 * t369;
            let t380 = t79 * t22;
            let t381 = t26 * t35;
            let t382 = t40 * t105;
            let t383 = t381 * t382;
            let t388 = -f64x8::splat(6.0) * t197 * t372 - t375;
            let t390 = t381 * t40;
            let t393 = t357 * t35;
            let t396 = v_tau * v_tau;
            let t397 = f64x8::splat(1.0) / t396;
            let t400 = ((t127).select(-t123 * t397 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t401 = t119 * t400;
            let t406 = f64x8::splat(146.0) / f64x8::splat(405.0) * t331 * t390 - f64x8::splat(73.0) / f64x8::splat(270.0) * t393 * t337 - f64x8::splat(146.0) / f64x8::splat(135.0) * t401 * t130 + f64x8::splat(146.0) / f64x8::splat(135.0) * t129 * t400;
            let t409 = t375 * t108 + f64x8::splat(6.0) * t197 * t377 - f64x8::splat(0.06288822469135802) * t380 * t383 + t388 * t134 + t240 * t406 / f64x8::splat(10.0);
            let t413 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t409));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t413;
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
