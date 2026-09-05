//! MGGA_X_TM fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`
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
pub fn mgga_x_tm_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
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
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = t22 * t23 / f64x8::splat(8.0);
            let t26 = (t25).simd_lt(f64x8::splat(1.0));
            let t27 = ((t26).select(t25, f64x8::splat(1.0)));
            let t28 = t27 * t27;
            let t29 = t28 * t27;
            let t31 = t28 + f64x8::splat(3.0) * t29;
            let t32 = f64x8::splat(1.0) + t29;
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t31 * t34;
            let t36 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t42 = f64x8::splat(M_CBRT2);
            let t43 = t42 * t42;
            let t44 = v_sigma * t43;
            let t45 = v_rho * v_rho;
            let t46 = t19 * t19;
            let t48 = f64x8::splat(1.0) / t46 / t45;
            let t49 = t44 * t48;
            let t50 = t41 * t49;
            let t52 = t36 * t36;
            let t54 = f64x8::splat(1.0) / t38 / t37;
            let t55 = t52 * t54;
            let t56 = v_sigma * v_sigma;
            let t57 = t56 * t42;
            let t58 = t45 * t45;
            let t59 = t58 * v_rho;
            let t61 = f64x8::splat(1.0) / t19 / t59;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t50 + f64x8::splat(0.00537989809245259) * t55 * t57 * t61;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t69 = v_tau * t43;
            let t71 = f64x8::splat(1.0) / t46 / v_rho;
            let t72 = t69 * t71;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t72 + f64x8::splat(0.256337604) * t52 * t39 + f64x8::splat(0.011867481666666667) * t49) * t36 * t40;
            let t82 = t66 * t66;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t66 + f64x8::splat(7.0) / f64x8::splat(9.0) * t81 * t83;
            let t88 = f64x8::splat(1.0) - t35;
            let t91 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t50) * t36;
            let t92 = t91 * t40;
            let t101 = (t72 - t49 / f64x8::splat(8.0)) * t36 * t40 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t50 / f64x8::splat(36.0);
            let t102 = t101 * t101;
            let t104 = t101 * t27;
            let t105 = f64x8::splat(1.0) - t27;
            let t108 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t92 * t49 + f64x8::splat(292.0) / f64x8::splat(405.0) * t102 - f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t105;
            let t109 = (simd::pow(t108, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t111 = t88 * t109 + t35 * t86;
            let t115 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t111));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
            let t117 = t18 / t46;
            let t121 = f64x8::splat(1.0) / t45;
            let t122 = v_sigma * t121;
            let t125 = ((t26).select(-t122 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t126 = t27 * t125;
            let t128 = t28 * t125;
            let t130 = f64x8::splat(2.0) * t126 + f64x8::splat(9.0) * t128;
            let t131 = t130 * t34;
            let t134 = f64x8::splat(1.0) / t33 / t32;
            let t135 = t31 * t134;
            let t136 = t86 * t28;
            let t137 = t136 * t125;
            let t141 = f64x8::splat(1.0) / t66 / t65;
            let t142 = t45 * v_rho;
            let t144 = f64x8::splat(1.0) / t46 / t142;
            let t145 = t44 * t144;
            let t146 = t41 * t145;
            let t148 = t58 * t45;
            let t150 = f64x8::splat(1.0) / t19 / t148;
            let t152 = t55 * t57 * t150;
            let t154 = -f64x8::splat(0.40121303703703703) * t146 - f64x8::splat(0.028692789826413812) * t152;
            let t158 = t69 * t48;
            let t165 = -f64x8::splat(0.17051554074074074) * t146 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t158 - f64x8::splat(0.031646617777777775) * t145) * t36 * t40;
            let t169 = f64x8::splat(1.0) / t82 / t65;
            let t170 = t81 * t169;
            let t173 = -t141 * t154 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t165 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t154;
            let t177 = f64x8::splat(6.0) * t135 * t128 - t131;
            let t179 = t109 * t109;
            let t180 = t179 * t179;
            let t181 = t180 * t180;
            let t182 = t181 * t109;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t88 * t183;
            let t195 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t158 + t145 / f64x8::splat(3.0)) * t36 * t40 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t146;
            let t198 = t195 * t27;
            let t201 = t101 * t125;
            let t206 = -f64x8::splat(125.0) / f64x8::splat(19683.0) * t152 - f64x8::splat(10.0) / f64x8::splat(9.0) * t92 * t145 + f64x8::splat(584.0) / f64x8::splat(405.0) * t101 * t195 - f64x8::splat(146.0) / f64x8::splat(135.0) * t198 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t201 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t125;
            let t209 = t131 * t86 - f64x8::splat(6.0) * t135 * t137 + t35 * t173 + t177 * t109 + t184 * t206 / f64x8::splat(10.0);
            let t214 = ((t3).select(f64x8::splat(0.0), -t7 * t117 * t111 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t209));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t214 + f64x8::splat(2.0) * t115;
            acc_vrho = tvrho0;
            let t219 = ((t26).select(t21 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t220 = t27 * t219;
            let t222 = t28 * t219;
            let t224 = f64x8::splat(2.0) * t220 + f64x8::splat(9.0) * t222;
            let t225 = t224 * t34;
            let t227 = t136 * t219;
            let t230 = t43 * t48;
            let t233 = v_sigma * t42;
            let t235 = t55 * t233 * t61;
            let t237 = f64x8::splat(0.1504548888888889) * t41 * t230 + f64x8::splat(0.01075979618490518) * t235;
            let t245 = -t141 * t237 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t41 * t230 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t237;
            let t249 = f64x8::splat(6.0) * t135 * t222 - t225;
            let t252 = t40 * t43;
            let t253 = t252 * t48;
            let t256 = t101 * t36;
            let t257 = t256 * t253;
            let t259 = t41 * t43;
            let t260 = t48 * t27;
            let t262 = t259 * t260 * t105;
            let t264 = t101 * t219;
            let t269 = f64x8::splat(125.0) / f64x8::splat(52488.0) * t235 + f64x8::splat(5.0) / f64x8::splat(12.0) * t91 * t253 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t257 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t262 - f64x8::splat(146.0) / f64x8::splat(135.0) * t264 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t219;
            let t272 = t225 * t86 - f64x8::splat(6.0) * t135 * t227 + t35 * t245 + t249 * t109 + t184 * t269 / f64x8::splat(10.0);
            let t276 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t272));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t276;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t278 = v_tau * v_tau;
            let t279 = f64x8::splat(1.0) / t278;
            let t282 = ((t26).select(-t22 * t279 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t283 = t27 * t282;
            let t285 = t28 * t282;
            let t287 = f64x8::splat(2.0) * t283 + f64x8::splat(9.0) * t285;
            let t288 = t287 * t34;
            let t290 = t136 * t282;
            let t293 = t35 * t43;
            let t294 = t71 * t36;
            let t295 = t40 * t83;
            let t296 = t294 * t295;
            let t301 = f64x8::splat(6.0) * t135 * t285 - t288;
            let t304 = t294 * t40;
            let t307 = t43 * t71;
            let t308 = t307 * t36;
            let t309 = t40 * t27;
            let t313 = t101 * t282;
            let t318 = f64x8::splat(146.0) / f64x8::splat(405.0) * t101 * t43 * t304 - f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t309 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t313 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t282;
            let t321 = t288 * t86 - f64x8::splat(6.0) * t135 * t290 - f64x8::splat(0.06288822469135802) * t293 * t296 + t301 * t109 + t184 * t318 / f64x8::splat(10.0);
            let t325 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t321));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t325;
            acc_vtau = tvtau0;
            let t328 = t18 * t71;
            let t335 = t125 * t125;
            let t337 = f64x8::splat(1.0) / t142;
            let t338 = v_sigma * t337;
            let t341 = ((t26).select(t338 * t23 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t344 = t27 * t335;
            let t346 = t28 * t341;
            let t348 = f64x8::splat(2.0) * t27 * t341 + f64x8::splat(2.0) * t335 + f64x8::splat(18.0) * t344 + f64x8::splat(9.0) * t346;
            let t349 = t348 * t34;
            let t351 = t130 * t134;
            let t356 = t33 * t33;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t31 * t357;
            let t359 = t28 * t28;
            let t360 = t86 * t359;
            let t361 = t360 * t335;
            let t364 = t173 * t28;
            let t365 = t364 * t125;
            let t368 = t86 * t27;
            let t369 = t368 * t335;
            let t372 = t136 * t341;
            let t375 = t65 * t65;
            let t377 = f64x8::splat(1.0) / t66 / t375;
            let t378 = t154 * t154;
            let t382 = f64x8::splat(1.0) / t46 / t58;
            let t383 = t44 * t382;
            let t384 = t41 * t383;
            let t386 = t58 * t142;
            let t388 = f64x8::splat(1.0) / t19 / t386;
            let t390 = t55 * t57 * t388;
            let t392 = f64x8::splat(1.4711144691358025) * t384 + f64x8::splat(0.18172100223395413) * t390;
            let t396 = t69 * t144;
            let t403 = f64x8::splat(0.625223649382716) * t384 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.6468503111111111) * t396 + f64x8::splat(0.11603759851851851) * t383) * t36 * t40;
            let t406 = t165 * t169;
            let t410 = f64x8::splat(1.0) / t82 / t375;
            let t411 = t81 * t410;
            let t416 = f64x8::splat(6.0) / f64x8::splat(25.0) * t377 * t378 - t141 * t392 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t403 * t83 - f64x8::splat(28.0) / f64x8::splat(45.0) * t406 * t154 + f64x8::splat(98.0) / f64x8::splat(225.0) * t411 * t378 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t392;
            let t420 = t359 * t335;
            let t427 = f64x8::splat(12.0) * t351 * t128 + f64x8::splat(12.0) * t135 * t344 + f64x8::splat(6.0) * t135 * t346 - f64x8::splat(54.0) * t358 * t420 - t349;
            let t429 = t177 * t183;
            let t433 = f64x8::splat(1.0) / t182 / t108;
            let t434 = t88 * t433;
            let t435 = t206 * t206;
            let t441 = t195 * t195;
            let t450 = (f64x8::splat(40.0) / f64x8::splat(9.0) * t396 - f64x8::splat(11.0) / f64x8::splat(9.0) * t383) * t36 * t40 / f64x8::splat(4.0) + f64x8::splat(22.0) / f64x8::splat(81.0) * t384;
            let t453 = t450 * t27;
            let t456 = t195 * t125;
            let t461 = t101 * t341;
            let t468 = f64x8::splat(125.0) / f64x8::splat(2187.0) * t390 + f64x8::splat(110.0) / f64x8::splat(27.0) * t92 * t383 + f64x8::splat(584.0) / f64x8::splat(405.0) * t441 + f64x8::splat(584.0) / f64x8::splat(405.0) * t101 * t450 - f64x8::splat(146.0) / f64x8::splat(135.0) * t453 * t105 - f64x8::splat(292.0) / f64x8::splat(135.0) * t456 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t198 * t125 - f64x8::splat(146.0) / f64x8::splat(135.0) * t461 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t101 * t335 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t341;
            let t471 = t349 * t86 - f64x8::splat(12.0) * t351 * t137 + f64x8::splat(2.0) * t131 * t173 + f64x8::splat(54.0) * t358 * t361 - f64x8::splat(12.0) * t135 * t365 - f64x8::splat(12.0) * t135 * t369 - f64x8::splat(6.0) * t135 * t372 + t35 * t416 + t427 * t109 + t429 * t206 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t435 + t184 * t468 / f64x8::splat(10.0);
            let t476 = ((t3).select(f64x8::splat(0.0), t7 * t328 * t111 / f64x8::splat(12.0) - t7 * t117 * t209 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t471));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t476 + f64x8::splat(4.0) * t214;
            acc_v2rho2 = tv2rho20;
            let t482 = t125 * t219;
            let t486 = ((t26).select(-t121 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t487 = t27 * t486;
            let t489 = t220 * t125;
            let t491 = t28 * t486;
            let t493 = f64x8::splat(2.0) * t482 + f64x8::splat(2.0) * t487 + f64x8::splat(18.0) * t489 + f64x8::splat(9.0) * t491;
            let t494 = t493 * t34;
            let t496 = t224 * t134;
            let t502 = t358 * t86;
            let t503 = t359 * t219;
            let t504 = t503 * t125;
            let t507 = t364 * t219;
            let t510 = t135 * t86;
            let t513 = t136 * t486;
            let t517 = t245 * t28;
            let t518 = t517 * t125;
            let t521 = t377 * t237;
            let t524 = t43 * t144;
            let t528 = t55 * t233 * t150;
            let t530 = -f64x8::splat(0.40121303703703703) * t41 * t524 - f64x8::splat(0.057385579652827624) * t528;
            let t536 = t48 * t169;
            let t542 = t237 * t154;
            let t547 = f64x8::splat(6.0) / f64x8::splat(25.0) * t521 * t154 - t141 * t530 / f64x8::splat(5.0) - f64x8::splat(0.11894873388203017) * t41 * t524 * t83 - f64x8::splat(0.017842310082304528) * t259 * t536 * t154 - f64x8::splat(14.0) / f64x8::splat(45.0) * t406 * t237 + f64x8::splat(98.0) / f64x8::splat(225.0) * t411 * t542 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t530;
            let t559 = f64x8::splat(6.0) * t496 * t128 + f64x8::splat(12.0) * t135 * t489 + f64x8::splat(6.0) * t135 * t491 + f64x8::splat(6.0) * t351 * t222 - f64x8::splat(54.0) * t358 * t504 - t494;
            let t561 = t249 * t183;
            let t566 = t269 * t206;
            let t570 = t252 * t144;
            let t573 = t195 * t36;
            let t574 = t573 * t253;
            let t576 = t256 * t570;
            let t578 = t144 * t27;
            let t580 = t259 * t578 * t105;
            let t584 = t259 * t48 * t125 * t105;
            let t587 = t259 * t260 * t125;
            let t589 = t195 * t219;
            let t592 = t101 * t486;
            let t601 = -f64x8::splat(125.0) / f64x8::splat(6561.0) * t528 - f64x8::splat(10.0) / f64x8::splat(9.0) * t91 * t570 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t574 + f64x8::splat(146.0) / f64x8::splat(10935.0) * t576 - f64x8::splat(73.0) / f64x8::splat(7290.0) * t580 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t584 - f64x8::splat(73.0) / f64x8::splat(19440.0) * t587 - f64x8::splat(146.0) / f64x8::splat(135.0) * t589 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t592 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t264 * t125 + f64x8::splat(146.0) / f64x8::splat(135.0) * t198 * t219 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t486;
            let t604 = t494 * t86 - f64x8::splat(6.0) * t496 * t137 + t225 * t173 - f64x8::splat(6.0) * t351 * t227 + f64x8::splat(54.0) * t502 * t504 - f64x8::splat(6.0) * t135 * t507 - f64x8::splat(12.0) * t510 * t489 - f64x8::splat(6.0) * t135 * t513 + t131 * t245 - f64x8::splat(6.0) * t135 * t518 + t35 * t547 + t559 * t109 + t561 * t206 / f64x8::splat(10.0) + t429 * t269 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t566 + t184 * t601 / f64x8::splat(10.0);
            let t609 = ((t3).select(f64x8::splat(0.0), -t7 * t117 * t272 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t604));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t609 + f64x8::splat(2.0) * t276;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t615 = t125 * t282;
            let t619 = ((t26).select(t122 * t279 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t620 = t27 * t619;
            let t622 = t283 * t125;
            let t624 = t28 * t619;
            let t626 = f64x8::splat(2.0) * t615 + f64x8::splat(2.0) * t620 + f64x8::splat(18.0) * t622 + f64x8::splat(9.0) * t624;
            let t627 = t626 * t34;
            let t629 = t287 * t134;
            let t635 = t359 * t282;
            let t636 = t635 * t125;
            let t639 = t364 * t282;
            let t644 = t136 * t619;
            let t647 = t131 * t43;
            let t650 = t135 * t307;
            let t651 = t83 * t28;
            let t653 = t41 * t651 * t125;
            let t657 = t48 * t36 * t295;
            let t660 = t35 * t307;
            let t662 = t41 * t169 * t154;
            let t675 = f64x8::splat(6.0) * t629 * t128 + f64x8::splat(12.0) * t135 * t622 + f64x8::splat(6.0) * t135 * t624 + f64x8::splat(6.0) * t351 * t285 - f64x8::splat(54.0) * t358 * t636 - t627;
            let t677 = t301 * t183;
            let t682 = t318 * t206;
            let t697 = t195 * t282;
            let t700 = t101 * t619;
            let t709 = f64x8::splat(146.0) / f64x8::splat(405.0) * t195 * t43 * t304 - f64x8::splat(146.0) / f64x8::splat(243.0) * t257 + f64x8::splat(73.0) / f64x8::splat(162.0) * t262 - f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t40 * t125 * t105 + f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t309 * t125 - f64x8::splat(146.0) / f64x8::splat(135.0) * t697 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t700 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t313 * t125 + f64x8::splat(146.0) / f64x8::splat(135.0) * t198 * t282 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t619;
            let t712 = t627 * t86 - f64x8::splat(6.0) * t629 * t137 + t288 * t173 - f64x8::splat(6.0) * t351 * t290 + f64x8::splat(54.0) * t502 * t636 - f64x8::splat(6.0) * t135 * t639 - f64x8::splat(12.0) * t510 * t622 - f64x8::splat(6.0) * t135 * t644 - f64x8::splat(0.06288822469135802) * t647 * t296 + f64x8::splat(0.37732934814814817) * t650 * t653 + f64x8::splat(0.10481370781893004) * t293 * t657 + f64x8::splat(0.02515528987654321) * t660 * t662 + t675 * t109 + t677 * t206 / f64x8::splat(10.0) + t429 * t318 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t682 + t184 * t709 / f64x8::splat(10.0);
            let t717 = ((t3).select(f64x8::splat(0.0), -t7 * t117 * t321 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t712));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t717 + f64x8::splat(2.0) * t325;
            acc_v2rhotau = tv2rhotau0;
            let t720 = t219 * t219;
            let t722 = ((t26).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t723 = t27 * t722;
            let t724 = f64x8::splat(2.0) * t723;
            let t725 = t27 * t720;
            let t727 = t28 * t722;
            let t728 = f64x8::splat(9.0) * t727;
            let t729 = f64x8::splat(2.0) * t720 + t724 + f64x8::splat(18.0) * t725 + t728;
            let t730 = t729 * t34;
            let t736 = t360 * t720;
            let t739 = t517 * t219;
            let t742 = t368 * t720;
            let t745 = t136 * t722;
            let t747 = f64x8::splat(6.0) * t135 * t745;
            let t748 = t237 * t237;
            let t751 = t141 * t52;
            let t752 = t54 * t42;
            let t753 = t752 * t61;
            let t761 = t170 * t52;
            let t764 = f64x8::splat(6.0) / f64x8::splat(25.0) * t377 * t748 - f64x8::splat(0.0021519592369810357) * t751 * t753 - f64x8::splat(0.035684620164609056) * t259 * t536 * t237 + f64x8::splat(98.0) / f64x8::splat(225.0) * t411 * t748 - f64x8::splat(0.0033474921464149445) * t761 * t753;
            let t768 = t359 * t720;
            let t774 = f64x8::splat(6.0) * t135 * t727;
            let t775 = f64x8::splat(12.0) * t135 * t725 + f64x8::splat(12.0) * t496 * t222 - f64x8::splat(54.0) * t358 * t768 - t730 + t774;
            let t779 = t269 * t269;
            let t782 = t42 * t61;
            let t783 = t55 * t782;
            let t785 = t48 * t219;
            let t787 = t259 * t785 * t105;
            let t790 = t259 * t260 * t219;
            let t792 = t101 * t722;
            let t794 = f64x8::splat(146.0) / f64x8::splat(135.0) * t792 * t105;
            let t798 = f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t722;
            let t799 = f64x8::splat(10073.0) / f64x8::splat(2099520.0) * t783 + f64x8::splat(73.0) / f64x8::splat(9720.0) * t787 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t790 - t794 + f64x8::splat(292.0) / f64x8::splat(135.0) * t101 * t720 + t798;
            let t802 = t730 * t86 - f64x8::splat(12.0) * t496 * t227 + f64x8::splat(2.0) * t225 * t245 + f64x8::splat(54.0) * t358 * t736 - f64x8::splat(12.0) * t135 * t739 - f64x8::splat(12.0) * t135 * t742 - t747 + t35 * t764 + t775 * t109 + t561 * t269 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t779 + t184 * t799 / f64x8::splat(10.0);
            let t806 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t802));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t806;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t808 = t219 * t282;
            let t812 = ((t26).select(-t21 * t279 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t813 = t27 * t812;
            let t815 = t283 * t219;
            let t817 = t28 * t812;
            let t819 = f64x8::splat(2.0) * t808 + f64x8::splat(2.0) * t813 + f64x8::splat(18.0) * t815 + f64x8::splat(9.0) * t817;
            let t820 = t819 * t34;
            let t827 = t635 * t219;
            let t830 = t517 * t282;
            let t835 = t136 * t812;
            let t838 = t225 * t43;
            let t842 = t41 * t651 * t219;
            let t846 = t41 * t169 * t237;
            let t859 = f64x8::splat(12.0) * t135 * t815 + f64x8::splat(6.0) * t135 * t817 + f64x8::splat(6.0) * t629 * t222 + f64x8::splat(6.0) * t496 * t285 - f64x8::splat(54.0) * t358 * t827 - t820;
            let t865 = t318 * t269;
            let t871 = t55 * t42 / t19 / t58;
            let t873 = t40 * t219;
            let t880 = t48 * t282;
            let t882 = t259 * t880 * t105;
            let t884 = t101 * t812;
            let t890 = t259 * t260 * t282;
            let t894 = -f64x8::splat(73.0) / f64x8::splat(29160.0) * t871 - f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t873 * t105 + f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t309 * t219 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t882 - f64x8::splat(146.0) / f64x8::splat(135.0) * t884 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t313 * t219 - f64x8::splat(73.0) / f64x8::splat(19440.0) * t890 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t812;
            let t897 = t820 * t86 - f64x8::splat(6.0) * t629 * t227 + t288 * t245 - f64x8::splat(6.0) * t496 * t290 + f64x8::splat(54.0) * t502 * t827 - f64x8::splat(6.0) * t135 * t830 - f64x8::splat(12.0) * t510 * t815 - f64x8::splat(6.0) * t135 * t835 - f64x8::splat(0.06288822469135802) * t838 * t296 + f64x8::splat(0.37732934814814817) * t650 * t842 + f64x8::splat(0.02515528987654321) * t660 * t846 + t859 * t109 + t677 * t269 / f64x8::splat(10.0) + t561 * t318 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t865 + t184 * t894 / f64x8::splat(10.0);
            let t901 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t897));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t901;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t903 = t282 * t282;
            let t906 = f64x8::splat(1.0) / t278 / v_tau;
            let t909 = ((t26).select(t22 * t906 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t910 = t27 * t909;
            let t912 = t27 * t903;
            let t914 = t28 * t909;
            let t916 = f64x8::splat(2.0) * t903 + f64x8::splat(2.0) * t910 + f64x8::splat(18.0) * t912 + f64x8::splat(9.0) * t914;
            let t917 = t916 * t34;
            let t921 = t288 * t43;
            let t924 = t360 * t903;
            let t928 = t41 * t651 * t282;
            let t931 = t368 * t903;
            let t934 = t136 * t909;
            let t939 = t359 * t903;
            let t946 = f64x8::splat(12.0) * t135 * t912 + f64x8::splat(6.0) * t135 * t914 + f64x8::splat(12.0) * t629 * t285 - f64x8::splat(54.0) * t358 * t939 - t917;
            let t950 = t318 * t318;
            let t958 = t40 * t282;
            let t965 = t101 * t909;
            let t972 = f64x8::splat(73.0) / f64x8::splat(405.0) * t42 / t19 / t142 * t55 - f64x8::splat(73.0) / f64x8::splat(135.0) * t308 * t958 * t105 + f64x8::splat(73.0) / f64x8::splat(135.0) * t308 * t309 * t282 - f64x8::splat(146.0) / f64x8::splat(135.0) * t965 * t105 + f64x8::splat(292.0) / f64x8::splat(135.0) * t101 * t903 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t909;
            let t975 = t917 * t86 - f64x8::splat(12.0) * t629 * t290 - f64x8::splat(0.12577644938271604) * t921 * t296 + f64x8::splat(54.0) * t358 * t924 + f64x8::splat(0.7546586962962963) * t650 * t928 - f64x8::splat(12.0) * t135 * t931 - f64x8::splat(6.0) * t135 * t934 + t946 * t109 + t677 * t318 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t434 * t950 + t184 * t972 / f64x8::splat(10.0);
            let t979 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t975));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t979;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
