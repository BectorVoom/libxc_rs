//! HYB_MGGA_X_M05 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
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
pub fn hyb_mgga_x_m05_fxc_unpol(
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
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_csi_HF = f64x8::splat(param_csi_HF);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_a_6 = f64x8::splat(param_a_6);
    let param_a_7 = f64x8::splat(param_a_7);
    let param_a_8 = f64x8::splat(param_a_8);
    let param_a_9 = f64x8::splat(param_a_9);
    let param_a_10 = f64x8::splat(param_a_10);
    let param_a_11 = f64x8::splat(param_a_11);
    let param_a_0 = f64x8::splat(param_a_0);
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
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = t20 * param_csi_HF;
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = t24 * t24;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t22 * t26;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_sigma * t29;
            let t31 = v_rho * v_rho;
            let t32 = t20 * t20;
            let t34 = f64x8::splat(1.0) / t32 / t31;
            let t38 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t27 * t30 * t34;
            let t41 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t38;
            let t43 = param_a_1;
            let t44 = t22 * t22;
            let t46 = f64x8::splat(3.0) / f64x8::splat(10.0) * t44 * t25;
            let t47 = v_tau * t29;
            let t49 = f64x8::splat(1.0) / t32 / v_rho;
            let t50 = t47 * t49;
            let t51 = t46 - t50;
            let t52 = t43 * t51;
            let t53 = t46 + t50;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = param_a_2;
            let t57 = t51 * t51;
            let t58 = t56 * t57;
            let t59 = t53 * t53;
            let t60 = f64x8::splat(1.0) / t59;
            let t62 = param_a_3;
            let t63 = t57 * t51;
            let t64 = t62 * t63;
            let t65 = t59 * t53;
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = param_a_4;
            let t69 = t57 * t57;
            let t70 = t68 * t69;
            let t71 = t59 * t59;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = param_a_5;
            let t75 = t69 * t51;
            let t76 = t74 * t75;
            let t77 = t71 * t53;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = param_a_6;
            let t81 = t69 * t57;
            let t82 = t80 * t81;
            let t83 = t71 * t59;
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = param_a_7;
            let t87 = t69 * t63;
            let t88 = t86 * t87;
            let t89 = t71 * t65;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = param_a_8;
            let t93 = t69 * t69;
            let t94 = t92 * t93;
            let t95 = t71 * t71;
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = param_a_9;
            let t99 = t93 * t51;
            let t100 = t98 * t99;
            let t102 = f64x8::splat(1.0) / t95 / t53;
            let t104 = param_a_10;
            let t105 = t93 * t57;
            let t106 = t104 * t105;
            let t108 = f64x8::splat(1.0) / t95 / t59;
            let t110 = param_a_11;
            let t112 = t110 * t93 * t63;
            let t114 = f64x8::splat(1.0) / t95 / t65;
            let t116 = t100 * t102 + t106 * t108 + t112 * t114 + t52 * t54 + t58 * t60 + t64 * t66 + t70 * t72 + t76 * t78 + t82 * t84 + t88 * t90 + t94 * t96 + param_a_0;
            let t117 = t41 * t116;
            let t121 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t117));
            let tzk0 = f64x8::splat(2.0) * t121;
            acc_zk = tzk0;
            let t123 = f64x8::splat(1.0) / t32 * param_csi_HF;
            let t127 = t4 * t18;
            let t128 = t31 * v_rho;
            let t130 = f64x8::splat(1.0) / t20 / t128;
            let t131 = t130 * param_csi_HF;
            let t132 = t38 * t38;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t127 * t131 * t133;
            let t137 = t27 * t30 * t116;
            let t140 = t43 * v_tau;
            let t145 = t52 * t60;
            let t146 = t47 * t34;
            let t149 = t56 * t51;
            let t150 = t149 * t60;
            let t153 = t58 * t66;
            let t156 = t62 * t57;
            let t157 = t156 * t66;
            let t160 = t64 * t72;
            let t163 = t68 * t63;
            let t164 = t163 * t72;
            let t167 = t70 * t78;
            let t170 = t74 * t69;
            let t171 = t170 * t78;
            let t174 = t76 * t84;
            let t177 = t80 * t75;
            let t178 = t177 * t84;
            let t181 = f64x8::splat(5.0) / f64x8::splat(3.0) * t140 * t29 * t34 * t54 + f64x8::splat(5.0) / f64x8::splat(3.0) * t145 * t146 + f64x8::splat(10.0) / f64x8::splat(3.0) * t150 * t146 + f64x8::splat(10.0) / f64x8::splat(3.0) * t153 * t146 + f64x8::splat(5.0) * t157 * t146 + f64x8::splat(5.0) * t160 * t146 + f64x8::splat(20.0) / f64x8::splat(3.0) * t164 * t146 + f64x8::splat(20.0) / f64x8::splat(3.0) * t167 * t146 + f64x8::splat(25.0) / f64x8::splat(3.0) * t171 * t146 + f64x8::splat(25.0) / f64x8::splat(3.0) * t174 * t146 + f64x8::splat(10.0) * t178 * t146;
            let t182 = t82 * t90;
            let t185 = t86 * t81;
            let t186 = t185 * t90;
            let t189 = t88 * t96;
            let t192 = t92 * t87;
            let t193 = t192 * t96;
            let t196 = t94 * t102;
            let t199 = t98 * t93;
            let t200 = t199 * t102;
            let t203 = t100 * t108;
            let t206 = t104 * t99;
            let t207 = t206 * t108;
            let t210 = t106 * t114;
            let t213 = t110 * t105;
            let t214 = t213 * t114;
            let t218 = f64x8::splat(1.0) / t95 / t71;
            let t219 = t112 * t218;
            let t222 = f64x8::splat(10.0) * t182 * t146 + f64x8::splat(35.0) / f64x8::splat(3.0) * t186 * t146 + f64x8::splat(35.0) / f64x8::splat(3.0) * t189 * t146 + f64x8::splat(40.0) / f64x8::splat(3.0) * t193 * t146 + f64x8::splat(40.0) / f64x8::splat(3.0) * t196 * t146 + f64x8::splat(15.0) * t200 * t146 + f64x8::splat(15.0) * t203 * t146 + f64x8::splat(50.0) / f64x8::splat(3.0) * t207 * t146 + f64x8::splat(50.0) / f64x8::splat(3.0) * t210 * t146 + f64x8::splat(55.0) / f64x8::splat(3.0) * t214 * t146 + f64x8::splat(55.0) / f64x8::splat(3.0) * t219 * t146;
            let t223 = t181 + t222;
            let t224 = t41 * t223;
            let t229 = ((t3).select(f64x8::splat(0.0), -t19 * t123 * t117 / f64x8::splat(8.0) + f64x8::splat(0.0040369036088841095) * t135 * t137 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t224));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t229 + f64x8::splat(2.0) * t121;
            acc_vrho = tvrho0;
            let t235 = t127 / t20 / t31 * param_csi_HF;
            let t236 = t133 * t22;
            let t237 = t26 * t29;
            let t239 = t236 * t237 * t116;
            let t242 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.0015138388533315413) * t235 * t239));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t242;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t244 = t43 * t29;
            let t247 = t60 * t29;
            let t248 = t247 * t49;
            let t252 = t66 * t29;
            let t253 = t252 * t49;
            let t258 = t72 * t29;
            let t259 = t258 * t49;
            let t264 = t78 * t29;
            let t265 = t264 * t49;
            let t270 = t84 * t29;
            let t271 = t270 * t49;
            let t276 = -t244 * t49 * t54 - f64x8::splat(2.0) * t149 * t248 - f64x8::splat(3.0) * t156 * t253 - f64x8::splat(4.0) * t163 * t259 - f64x8::splat(5.0) * t170 * t265 - f64x8::splat(6.0) * t177 * t271 - t52 * t248 - f64x8::splat(2.0) * t58 * t253 - f64x8::splat(3.0) * t64 * t259 - f64x8::splat(4.0) * t70 * t265 - f64x8::splat(5.0) * t76 * t271;
            let t277 = t90 * t29;
            let t278 = t277 * t49;
            let t283 = t96 * t29;
            let t284 = t283 * t49;
            let t289 = t102 * t29;
            let t290 = t289 * t49;
            let t295 = t108 * t29;
            let t296 = t295 * t49;
            let t301 = t114 * t29;
            let t302 = t301 * t49;
            let t307 = t218 * t29;
            let t311 = -f64x8::splat(11.0) * t112 * t307 * t49 - f64x8::splat(9.0) * t100 * t296 - f64x8::splat(10.0) * t106 * t302 - f64x8::splat(7.0) * t185 * t278 - f64x8::splat(8.0) * t192 * t284 - f64x8::splat(9.0) * t199 * t290 - f64x8::splat(10.0) * t206 * t296 - f64x8::splat(11.0) * t213 * t302 - f64x8::splat(6.0) * t82 * t278 - f64x8::splat(7.0) * t88 * t284 - f64x8::splat(8.0) * t94 * t290;
            let t312 = t276 + t311;
            let t313 = t41 * t312;
            let t317 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t313));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t317;
            acc_vtau = tvtau0;
            let t320 = t49 * param_csi_HF;
            let t324 = t31 * t31;
            let t326 = f64x8::splat(1.0) / t20 / t324;
            let t327 = t326 * param_csi_HF;
            let t329 = t127 * t327 * t133;
            let t335 = t324 * t128;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t336 * param_csi_HF;
            let t339 = f64x8::splat(1.0) / t132 / t38;
            let t341 = t127 * t337 * t339;
            let t343 = f64x8::splat(1.0) / t24 / t23;
            let t344 = t44 * t343;
            let t345 = v_sigma * v_sigma;
            let t346 = t345 * t28;
            let t348 = t344 * t346 * t116;
            let t352 = t27 * t30 * t223;
            let t356 = f64x8::splat(1.0) / t32 / t128;
            let t361 = v_tau * v_tau;
            let t362 = t43 * t361;
            let t363 = t324 * v_rho;
            let t365 = f64x8::splat(1.0) / t20 / t363;
            let t366 = t28 * t365;
            let t367 = t366 * t60;
            let t370 = t56 * t361;
            let t373 = t82 * t96;
            let t374 = t361 * t28;
            let t375 = t374 * t365;
            let t378 = t86 * t75;
            let t379 = t378 * t90;
            let t382 = t185 * t96;
            let t385 = t88 * t102;
            let t388 = t92 * t81;
            let t389 = t388 * t96;
            let t392 = t192 * t102;
            let t395 = t94 * t108;
            let t398 = t98 * t87;
            let t399 = t398 * t102;
            let t402 = t199 * t108;
            let t405 = t149 * t66;
            let t408 = -f64x8::splat(40.0) / f64x8::splat(9.0) * t140 * t29 * t356 * t54 + f64x8::splat(100.0) / f64x8::splat(9.0) * t362 * t367 + f64x8::splat(100.0) / f64x8::splat(9.0) * t370 * t367 + f64x8::splat(700.0) / f64x8::splat(3.0) * t373 * t375 + f64x8::splat(700.0) / f64x8::splat(3.0) * t379 * t375 + f64x8::splat(4900.0) / f64x8::splat(9.0) * t382 * t375 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t385 * t375 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t389 * t375 + f64x8::splat(6400.0) / f64x8::splat(9.0) * t392 * t375 + f64x8::splat(400.0) * t395 * t375 + f64x8::splat(400.0) * t399 * t375 + f64x8::splat(900.0) * t402 * t375 + f64x8::splat(400.0) / f64x8::splat(9.0) * t405 * t375;
            let t409 = t58 * t72;
            let t412 = t62 * t51;
            let t413 = t412 * t66;
            let t416 = t156 * t72;
            let t419 = t64 * t78;
            let t422 = t68 * t57;
            let t423 = t422 * t72;
            let t426 = t163 * t78;
            let t429 = t70 * t84;
            let t432 = t74 * t63;
            let t433 = t432 * t78;
            let t436 = t170 * t84;
            let t439 = t76 * t90;
            let t442 = t52 * t66;
            let t445 = t47 * t356;
            let t452 = f64x8::splat(100.0) / f64x8::splat(3.0) * t409 * t375 + f64x8::splat(100.0) / f64x8::splat(3.0) * t413 * t375 + f64x8::splat(100.0) * t416 * t375 + f64x8::splat(200.0) / f64x8::splat(3.0) * t419 * t375 + f64x8::splat(200.0) / f64x8::splat(3.0) * t423 * t375 + f64x8::splat(1600.0) / f64x8::splat(9.0) * t426 * t375 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t429 * t375 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t433 * t375 + f64x8::splat(2500.0) / f64x8::splat(9.0) * t436 * t375 + f64x8::splat(500.0) / f64x8::splat(3.0) * t439 * t375 + f64x8::splat(100.0) / f64x8::splat(9.0) * t442 * t375 - f64x8::splat(400.0) / f64x8::splat(9.0) * t210 * t445 - f64x8::splat(440.0) / f64x8::splat(9.0) * t214 * t445 - f64x8::splat(440.0) / f64x8::splat(9.0) * t219 * t445;
            let t480 = -f64x8::splat(80.0) / f64x8::splat(3.0) * t182 * t445 - f64x8::splat(280.0) / f64x8::splat(9.0) * t186 * t445 - f64x8::splat(280.0) / f64x8::splat(9.0) * t189 * t445 - f64x8::splat(320.0) / f64x8::splat(9.0) * t193 * t445 - f64x8::splat(320.0) / f64x8::splat(9.0) * t196 * t445 - f64x8::splat(40.0) * t200 * t445 - f64x8::splat(40.0) * t203 * t445 - f64x8::splat(400.0) / f64x8::splat(9.0) * t207 * t445 - f64x8::splat(80.0) / f64x8::splat(9.0) * t153 * t445 - f64x8::splat(40.0) / f64x8::splat(3.0) * t157 * t445 - f64x8::splat(40.0) / f64x8::splat(3.0) * t160 * t445 - f64x8::splat(160.0) / f64x8::splat(9.0) * t164 * t445 - f64x8::splat(160.0) / f64x8::splat(9.0) * t167 * t445;
            let t491 = t100 * t114;
            let t494 = t104 * t93;
            let t495 = t494 * t108;
            let t498 = t206 * t114;
            let t501 = t106 * t218;
            let t504 = t110 * t99;
            let t505 = t504 * t114;
            let t508 = t213 * t218;
            let t512 = f64x8::splat(1.0) / t95 / t77;
            let t513 = t112 * t512;
            let t516 = t80 * t69;
            let t517 = t516 * t84;
            let t520 = t177 * t90;
            let t523 = -f64x8::splat(200.0) / f64x8::splat(9.0) * t171 * t445 - f64x8::splat(200.0) / f64x8::splat(9.0) * t174 * t445 - f64x8::splat(80.0) / f64x8::splat(3.0) * t178 * t445 - f64x8::splat(40.0) / f64x8::splat(9.0) * t145 * t445 - f64x8::splat(80.0) / f64x8::splat(9.0) * t150 * t445 + f64x8::splat(500.0) * t491 * t375 + f64x8::splat(500.0) * t495 * t375 + f64x8::splat(10000.0) / f64x8::splat(9.0) * t498 * t375 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t501 * t375 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t505 * t375 + f64x8::splat(12100.0) / f64x8::splat(9.0) * t508 * t375 + f64x8::splat(2200.0) / f64x8::splat(3.0) * t513 * t375 + f64x8::splat(500.0) / f64x8::splat(3.0) * t517 * t375 + f64x8::splat(400.0) * t520 * t375;
            let t525 = t408 + t452 + t480 + t523;
            let t526 = t41 * t525;
            let t531 = ((t3).select(f64x8::splat(0.0), t19 * t320 * t117 / f64x8::splat(12.0) - f64x8::splat(0.01211071082665233) * t329 * t137 - t19 * t123 * t224 / f64x8::splat(4.0) + f64x8::splat(0.0003938492381143005) * t341 * t348 + f64x8::splat(0.008073807217768219) * t135 * t352 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t526));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t531 + f64x8::splat(4.0) * t229;
            acc_v2rho2 = tv2rho20;
            let t534 = t127 * t131;
            let t537 = t324 * t31;
            let t538 = f64x8::splat(1.0) / t537;
            let t539 = t538 * param_csi_HF;
            let t541 = t127 * t539 * t339;
            let t544 = t344 * t28 * t116 * v_sigma;
            let t548 = t236 * t237 * t223;
            let t552 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.003532290657773596) * t534 * t239 - f64x8::splat(0.00014769346429286268) * t541 * t544 - f64x8::splat(0.0015138388533315413) * t235 * t548));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t552 + f64x8::splat(2.0) * t242;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t559 = t27 * t30 * t312;
            let t565 = t56 * v_tau;
            let t566 = t28 * t326;
            let t570 = t301 * t34;
            let t576 = t283 * t34;
            let t581 = t289 * t34;
            let t586 = t295 * t34;
            let t593 = t264 * t34;
            let t598 = f64x8::splat(5.0) / f64x8::splat(3.0) * t244 * t34 * t54 - f64x8::splat(20.0) / f64x8::splat(3.0) * t565 * t566 * t60 + f64x8::splat(55.0) / f64x8::splat(3.0) * t213 * t570 + f64x8::splat(55.0) / f64x8::splat(3.0) * t112 * t307 * t34 + f64x8::splat(35.0) / f64x8::splat(3.0) * t88 * t576 + f64x8::splat(40.0) / f64x8::splat(3.0) * t192 * t576 + f64x8::splat(40.0) / f64x8::splat(3.0) * t94 * t581 + f64x8::splat(15.0) * t199 * t581 + f64x8::splat(15.0) * t100 * t586 + f64x8::splat(50.0) / f64x8::splat(3.0) * t206 * t586 + f64x8::splat(50.0) / f64x8::splat(3.0) * t106 * t570 + f64x8::splat(20.0) / f64x8::splat(3.0) * t70 * t593 + f64x8::splat(25.0) / f64x8::splat(3.0) * t170 * t593;
            let t599 = t270 * t34;
            let t604 = t277 * t34;
            let t609 = t247 * t34;
            let t614 = t252 * t34;
            let t619 = t258 * t34;
            let t624 = t43 * t28;
            let t625 = t326 * t60;
            let t629 = t566 * v_tau;
            let t636 = f64x8::splat(25.0) / f64x8::splat(3.0) * t76 * t599 + f64x8::splat(10.0) * t177 * t599 + f64x8::splat(10.0) * t82 * t604 + f64x8::splat(35.0) / f64x8::splat(3.0) * t185 * t604 + f64x8::splat(5.0) / f64x8::splat(3.0) * t52 * t609 + f64x8::splat(10.0) / f64x8::splat(3.0) * t149 * t609 + f64x8::splat(10.0) / f64x8::splat(3.0) * t58 * t614 + f64x8::splat(5.0) * t156 * t614 + f64x8::splat(5.0) * t64 * t619 + f64x8::splat(20.0) / f64x8::splat(3.0) * t163 * t619 - f64x8::splat(20.0) / f64x8::splat(3.0) * t624 * t625 * v_tau - f64x8::splat(60.0) * t416 * t629 - f64x8::splat(40.0) * t419 * t629 - f64x8::splat(100.0) * t439 * t629;
            let t664 = -f64x8::splat(100.0) * t517 * t629 - f64x8::splat(240.0) * t520 * t629 - f64x8::splat(140.0) * t373 * t629 - f64x8::splat(140.0) * t379 * t629 - f64x8::splat(980.0) / f64x8::splat(3.0) * t382 * t629 - f64x8::splat(20.0) / f64x8::splat(3.0) * t442 * t629 - f64x8::splat(80.0) / f64x8::splat(3.0) * t405 * t629 - f64x8::splat(20.0) * t409 * t629 - f64x8::splat(20.0) * t413 * t629 - f64x8::splat(440.0) * t513 * t629 - f64x8::splat(560.0) / f64x8::splat(3.0) * t385 * t629 - f64x8::splat(560.0) / f64x8::splat(3.0) * t389 * t629 - f64x8::splat(1280.0) / f64x8::splat(3.0) * t392 * t629;
            let t693 = -f64x8::splat(240.0) * t395 * t629 - f64x8::splat(240.0) * t399 * t629 - f64x8::splat(540.0) * t402 * t629 - f64x8::splat(300.0) * t491 * t629 - f64x8::splat(300.0) * t495 * t629 - f64x8::splat(2000.0) / f64x8::splat(3.0) * t498 * t629 - f64x8::splat(40.0) * t423 * t629 - f64x8::splat(320.0) / f64x8::splat(3.0) * t426 * t629 - f64x8::splat(200.0) / f64x8::splat(3.0) * t429 * t629 - f64x8::splat(200.0) / f64x8::splat(3.0) * t433 * t629 - f64x8::splat(500.0) / f64x8::splat(3.0) * t436 * t629 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t501 * t629 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t505 * t629 - f64x8::splat(2420.0) / f64x8::splat(3.0) * t508 * t629;
            let t695 = t598 + t636 + t664 + t693;
            let t696 = t41 * t695;
            let t701 = ((t3).select(f64x8::splat(0.0), -t19 * t123 * t313 / f64x8::splat(8.0) + f64x8::splat(0.0040369036088841095) * t135 * t559 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t696));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t701 + f64x8::splat(2.0) * t317;
            acc_v2rhotau = tv2rhotau0;
            let t704 = f64x8::splat(1.0) / t363;
            let t706 = t127 * t704 * param_csi_HF;
            let t707 = t339 * t44;
            let t708 = t343 * t28;
            let t710 = t707 * t708 * t116;
            let t713 = ((t3).select(f64x8::splat(0.0), f64x8::splat(5.538504910982351e-05) * t706 * t710));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t713;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t716 = t236 * t237 * t312;
            let t719 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.0015138388533315413) * t235 * t716));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t719;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t721 = t56 * t28;
            let t722 = t130 * t60;
            let t727 = t108 * t28;
            let t728 = t727 * t130;
            let t731 = t114 * t28;
            let t732 = t731 * t130;
            let t739 = t218 * t28;
            let t740 = t739 * t130;
            let t747 = t512 * t28;
            let t751 = t78 * t28;
            let t752 = t751 * t130;
            let t755 = t72 * t28;
            let t756 = t755 * t130;
            let t761 = t84 * t28;
            let t762 = t761 * t130;
            let t769 = f64x8::splat(264.0) * t112 * t747 * t130 + f64x8::splat(180.0) * t100 * t732 + f64x8::splat(220.0) * t106 * t740 + f64x8::splat(64.0) * t163 * t752 + f64x8::splat(100.0) * t170 * t762 + f64x8::splat(324.0) * t199 * t728 + f64x8::splat(400.0) * t206 * t732 + f64x8::splat(484.0) * t213 * t740 + f64x8::splat(24.0) * t422 * t756 + f64x8::splat(40.0) * t432 * t752 + f64x8::splat(180.0) * t494 * t728 + f64x8::splat(220.0) * t504 * t732 + f64x8::splat(4.0) * t624 * t722 + f64x8::splat(24.0) * t64 * t752 + f64x8::splat(40.0) * t70 * t762 + f64x8::splat(4.0) * t721 * t722;
            let t770 = t90 * t28;
            let t771 = t770 * t130;
            let t778 = t96 * t28;
            let t779 = t778 * t130;
            let t786 = t102 * t28;
            let t787 = t786 * t130;
            let t798 = t66 * t28;
            let t799 = t798 * t130;
            let t810 = f64x8::splat(16.0) * t149 * t799 + f64x8::splat(36.0) * t156 * t756 + f64x8::splat(144.0) * t177 * t771 + f64x8::splat(196.0) * t185 * t779 + f64x8::splat(256.0) * t192 * t787 + f64x8::splat(84.0) * t378 * t771 + f64x8::splat(112.0) * t388 * t779 + f64x8::splat(144.0) * t398 * t787 + f64x8::splat(12.0) * t412 * t799 + f64x8::splat(60.0) * t516 * t762 + f64x8::splat(4.0) * t52 * t799 + f64x8::splat(12.0) * t58 * t756 + f64x8::splat(144.0) * t94 * t728 + f64x8::splat(60.0) * t76 * t771 + f64x8::splat(84.0) * t82 * t779 + f64x8::splat(112.0) * t88 * t787;
            let t811 = t769 + t810;
            let t812 = t41 * t811;
            let t816 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t21 * t812));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t816;
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
