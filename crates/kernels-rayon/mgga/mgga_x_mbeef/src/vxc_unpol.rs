//! MGGA_X_MBEEF vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeef.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeef_vxc_unpol(
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
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = t26 * v_sigma;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = v_sigma * t29;
            let t36 = t35 * t33;
            let t39 = f64x8::splat(6.5124) + t26 * t36 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t34 * t40;
            let t42 = t27 * t41;
            let t44 = v_tau * t29;
            let t46 = f64x8::splat(1.0) / t31 / v_rho;
            let t52 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t44 * t46 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t53 = (f64x8::splat(10000.0)).simd_le(t52);
            let t54 = (f64x8::splat(10000.0)).simd_lt(t52);
            let t55 = ((t54).select(t52, f64x8::splat(10000.0)));
            let t56 = t55 * t55;
            let t59 = t56 * t55;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t56 * t56;
            let t62 = f64x8::splat(1.0) / t61;
            let t65 = ((t54).select(f64x8::splat(10000.0), t52));
            let t66 = t65 * t65;
            let t67 = f64x8::splat(1.0) - t66;
            let t68 = t67 * t67;
            let t69 = t68 * t67;
            let t70 = t66 * t65;
            let t71 = f64x8::splat(1.0) + t70;
            let t73 = t70 * t71 + f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = ((t53).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t56 - t60 + f64x8::splat(3.0) * t62, -t69 * t74));
            let t77 = t76 * t76;
            let t78 = t77 * t76;
            let t79 = t77 * t77;
            let t80 = t79 * t78;
            let t83 = t42 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t84 = t83 * t83;
            let t85 = t84 * t83;
            let t87 = t84 * t84;
            let t88 = t87 * t84;
            let t91 = t87 * t85;
            let t93 = t87 * t83;
            let t97 = f64x8::splat(429.0) / f64x8::splat(16.0) * t91 - f64x8::splat(693.0) / f64x8::splat(16.0) * t93 + f64x8::splat(315.0) / f64x8::splat(16.0) * t85 - f64x8::splat(35.0) / f64x8::splat(192.0) * t42 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t99 = t79 * t76;
            let t103 = f64x8::splat(429.0) / f64x8::splat(16.0) * t80 - f64x8::splat(693.0) / f64x8::splat(16.0) * t99 + f64x8::splat(315.0) / f64x8::splat(16.0) * t78 - f64x8::splat(35.0) / f64x8::splat(16.0) * t76;
            let t106 = t79 * t77;
            let t110 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t106 - f64x8::splat(315.0) / f64x8::splat(16.0) * t79 + f64x8::splat(105.0) / f64x8::splat(16.0) * t77;
            let t116 = f64x8::splat(63.0) / f64x8::splat(8.0) * t99 - f64x8::splat(35.0) / f64x8::splat(4.0) * t78 + f64x8::splat(15.0) / f64x8::splat(8.0) * t76;
            let t121 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t79 - f64x8::splat(15.0) / f64x8::splat(4.0) * t77;
            let t126 = f64x8::splat(5.0) / f64x8::splat(2.0) * t78 - f64x8::splat(3.0) / f64x8::splat(2.0) * t76;
            let t130 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t77;
            let t133 = t97 * t76;
            let t139 = f64x8::splat(63.0) / f64x8::splat(8.0) * t93 - f64x8::splat(35.0) / f64x8::splat(4.0) * t85 + f64x8::splat(5.0) / f64x8::splat(32.0) * t42 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t146 = -f64x8::splat(0.013022208355989584) * t42 + f64x8::splat(1.9735677658125e-05) * t80 + f64x8::splat(0.497944638409375) * t85 + f64x8::splat(0.080024660533125) * t88 - f64x8::splat(0.004373652639371875) * t76 + f64x8::splat(8.88525527e-09) * t97 * t103 - f64x8::splat(7.74224962e-09) * t97 * t110 - f64x8::splat(3.38128188e-08) * t97 * t116 + f64x8::splat(5.54588743e-08) * t97 * t121 + f64x8::splat(5.05920757e-08) * t97 * t126 - f64x8::splat(2.7652468e-07) * t97 * t130 + f64x8::splat(0.00940675747) * t133 - f64x8::splat(0.138056183978125) * t87 - f64x8::splat(1.38472194e-08) * t139 * t110 - f64x8::splat(3.76702959e-08) * t139 * t116 + f64x8::splat(1.62238741e-07) * t139 * t121;
            let t151 = t139 * t76;
            let t155 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t87 - f64x8::splat(15.0) / f64x8::splat(4.0) * t84;
            let t168 = t155 * t76;
            let t172 = f64x8::splat(5.0) / f64x8::splat(2.0) * t85 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t185 = -f64x8::splat(0.00896771404) * t139 * t126 - f64x8::splat(0.0188495102) * t139 * t130 - f64x8::splat(0.00884148272) * t151 - f64x8::splat(4.93824365e-09) * t155 * t103 + f64x8::splat(9.12223751e-09) * t155 * t110 + f64x8::splat(2.09603871e-08) * t155 * t116 - f64x8::splat(7.90811707e-08) * t155 * t121 + f64x8::splat(0.00631891628) * t155 * t126 - f64x8::splat(0.0182911291) * t155 * t130 + f64x8::splat(0.0162638575) * t168 + f64x8::splat(6.74910119e-09) * t172 * t103 - f64x8::splat(2.16860568e-08) * t172 * t110 + f64x8::splat(0.000896739466) * t172 * t116 + f64x8::splat(0.00339308972) * t172 * t121 - f64x8::splat(0.00845508103) * t172 * t126 + f64x8::splat(0.0280678872) * t172 * t130;
            let t187 = t172 * t76;
            let t190 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t84;
            let t206 = t190 * t76;
            let t218 = -f64x8::splat(0.0182177954) * t187 - f64x8::splat(2.23014657e-09) * t190 * t103 - f64x8::splat(0.395061199588125) * t93 - f64x8::splat(0.000945883103563125) * t99 + f64x8::splat(0.004646102821846875) * t78 + f64x8::splat(6.68980219e-09) * t190 * t110 - f64x8::splat(0.00035104103) * t190 * t116 + f64x8::splat(0.00182906057) * t190 * t121 + f64x8::splat(0.00293253041) * t190 * t126 - f64x8::splat(0.0150103636) * t190 * t130 - f64x8::splat(0.043464346) * t206 - f64x8::splat(9.40351563e-06) * t83 * t103 - f64x8::splat(5.14204676e-05) * t83 * t110 + f64x8::splat(0.000822139896) * t83 * t116 + f64x8::splat(0.00119130546) * t83 * t121 - f64x8::splat(0.00303347141) * t83 * t126;
            let t221 = t83 * t76;
            let t226 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t88 - f64x8::splat(315.0) / f64x8::splat(16.0) * t87 + f64x8::splat(105.0) / f64x8::splat(16.0) * t84;
            let t239 = t226 * t76;
            let t248 = f64x8::splat(1.3805672252189969) - f64x8::splat(0.00879090772) * t83 * t130 + f64x8::splat(0.100339208) * t221 - f64x8::splat(6.91592964e-09) * t226 * t103 + f64x8::splat(6.94482484e-09) * t226 * t110 + f64x8::splat(2.36391411e-08) * t226 * t116 - f64x8::splat(4.16393106e-08) * t226 * t121 - f64x8::splat(2.65114646e-08) * t226 * t126 + f64x8::splat(1.69805915e-07) * t226 * t130 - f64x8::splat(0.00957417512) * t239 + f64x8::splat(8.50272392e-09) * t139 * t103 + f64x8::splat(0.106025815520625) * t91 - f64x8::splat(8.0008813355625e-05) * t106 + f64x8::splat(0.003020715669803125) * t79 + f64x8::splat(0.007031826877565625) * t77 - f64x8::splat(0.092294814328125) * t84;
            let t250 = t146 + t185 + t218 + t248;
            let t254 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t250));
            let tzk0 = f64x8::splat(2.0) * t254;
            acc_zk = tzk0;
            let t256 = t18 / t31;
            let t262 = t30 * v_rho;
            let t264 = f64x8::splat(1.0) / t31 / t262;
            let t270 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * t44 * t33 + t35 * t264 / f64x8::splat(3.0)) * t21 * t25;
            let t271 = ((t54).select(t270, f64x8::splat(0.0)));
            let t274 = t62 * t271;
            let t277 = f64x8::splat(1.0) / t61 / t55;
            let t278 = t277 * t271;
            let t281 = t68 * t74;
            let t282 = ((t54).select(f64x8::splat(0.0), t270));
            let t283 = t65 * t282;
            let t286 = t73 * t73;
            let t287 = f64x8::splat(1.0) / t286;
            let t288 = t69 * t287;
            let t289 = t66 * t71;
            let t291 = t66 * t66;
            let t292 = t291 * t65;
            let t295 = f64x8::splat(3.0) * t289 * t282 + f64x8::splat(3.0) * t292 * t282;
            let t298 = ((t53).select(f64x8::splat(6.0) * t60 * t271 + f64x8::splat(3.0) * t274 - f64x8::splat(12.0) * t278, f64x8::splat(6.0) * t281 * t283 + t288 * t295));
            let t301 = t29 * t264;
            let t302 = t301 * t40;
            let t303 = t27 * t302;
            let t305 = t21 * t21;
            let t307 = f64x8::splat(1.0) / t23 / t22;
            let t308 = t305 * t307;
            let t309 = v_sigma * v_sigma;
            let t310 = t308 * t309;
            let t311 = t30 * t30;
            let t312 = t311 * t30;
            let t314 = f64x8::splat(1.0) / t19 / t312;
            let t316 = t39 * t39;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t28 * t314 * t317;
            let t319 = t310 * t318;
            let t321 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t303 + t319 / f64x8::splat(54.0);
            let t322 = t83 * t321;
            let t349 = -f64x8::splat(0.02637272316) * t221 * t298 + f64x8::splat(0.00548718171) * t322 * t121 + f64x8::splat(0.00879759123) * t322 * t126 - f64x8::splat(0.0450310908) * t322 * t130 - f64x8::splat(0.0450310908) * t206 * t298 - f64x8::splat(0.130393038) * t322 * t76 + f64x8::splat(0.0842036616) * t187 * t298 - f64x8::splat(6.69043971e-09) * t322 * t103 + f64x8::splat(2.006940657e-08) * t322 * t110 - f64x8::splat(0.00105312309) * t322 * t116 - f64x8::splat(0.0548733873) * t168 * t298 - f64x8::splat(0.0565485306) * t151 * t298 + f64x8::splat(5.09417745e-07) * t239 * t298 - f64x8::splat(8.2957404e-07) * t133 * t298;
            let t350 = t99 * t298;
            let t352 = t78 * t298;
            let t354 = t76 * t298;
            let t356 = f64x8::splat(693.0) / f64x8::splat(8.0) * t350 - f64x8::splat(315.0) / f64x8::splat(4.0) * t352 + f64x8::splat(105.0) / f64x8::splat(8.0) * t354;
            let t359 = t84 * t321;
            let t364 = f64x8::splat(15.0) / f64x8::splat(2.0) * t359 + t303 / f64x8::splat(3.0) - t319 / f64x8::splat(36.0);
            let t365 = t364 * t76;
            let t367 = t172 * t298;
            let t369 = t106 * t298;
            let t371 = t79 * t298;
            let t373 = t77 * t298;
            let t376 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t369 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t371 + f64x8::splat(945.0) / f64x8::splat(16.0) * t373 - f64x8::splat(35.0) / f64x8::splat(16.0) * t298;
            let t383 = f64x8::splat(15.0) / f64x8::splat(2.0) * t373 - f64x8::splat(3.0) / f64x8::splat(2.0) * t298;
            let t393 = f64x8::splat(315.0) / f64x8::splat(8.0) * t371 - f64x8::splat(105.0) / f64x8::splat(4.0) * t373 + f64x8::splat(15.0) / f64x8::splat(8.0) * t298;
            let t398 = t93 * t321;
            let t400 = t85 * t321;
            let t403 = f64x8::splat(693.0) / f64x8::splat(8.0) * t398 - f64x8::splat(315.0) / f64x8::splat(4.0) * t400 + f64x8::splat(105.0) / f64x8::splat(8.0) * t322;
            let t410 = f64x8::splat(6.68980219e-09) * t190 * t356 + f64x8::splat(1.493833915228125) * t359 - f64x8::splat(0.0182177954) * t365 - f64x8::splat(0.0182177954) * t367 - f64x8::splat(2.23014657e-09) * t190 * t376 - f64x8::splat(0.00845508103) * t364 * t126 - f64x8::splat(0.00845508103) * t172 * t383 + f64x8::splat(0.0280678872) * t364 * t130 + f64x8::splat(0.000896739466) * t364 * t116 + f64x8::splat(0.000896739466) * t172 * t393 + f64x8::splat(0.00339308972) * t364 * t121 - f64x8::splat(2.65114646e-08) * t403 * t126 + f64x8::splat(2.36391411e-08) * t403 * t116 + f64x8::splat(2.36391411e-08) * t226 * t393;
            let t416 = t88 * t321;
            let t420 = f64x8::splat(35.0) / f64x8::splat(2.0) * t400 - f64x8::splat(15.0) / f64x8::splat(2.0) * t322;
            let t431 = f64x8::splat(35.0) / f64x8::splat(2.0) * t352 - f64x8::splat(15.0) / f64x8::splat(2.0) * t354;
            let t438 = t87 * t321;
            let t444 = f64x8::splat(315.0) / f64x8::splat(8.0) * t438 - f64x8::splat(105.0) / f64x8::splat(4.0) * t359 - f64x8::splat(5.0) / f64x8::splat(12.0) * t303 + f64x8::splat(5.0) / f64x8::splat(144.0) * t319;
            let t451 = f64x8::splat(6.94482484e-09) * t403 * t110 + f64x8::splat(6.94482484e-09) * t226 * t356 + f64x8::splat(0.742180708644375) * t416 - f64x8::splat(0.0182911291) * t420 * t130 + f64x8::splat(2.09603871e-08) * t420 * t116 + f64x8::splat(2.09603871e-08) * t155 * t393 - f64x8::splat(7.90811707e-08) * t420 * t121 - f64x8::splat(7.90811707e-08) * t155 * t431 + f64x8::splat(9.12223751e-09) * t420 * t110 + f64x8::splat(9.12223751e-09) * t155 * t356 - f64x8::splat(1.975305997940625) * t438 + f64x8::splat(1.62238741e-07) * t444 * t121 + f64x8::splat(1.62238741e-07) * t139 * t431 - f64x8::splat(1.38472194e-08) * t444 * t110;
            let t456 = t226 * t298;
            let t465 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t416 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t438 + f64x8::splat(945.0) / f64x8::splat(16.0) * t359 + f64x8::splat(35.0) / f64x8::splat(72.0) * t303 - f64x8::splat(35.0) / f64x8::splat(864.0) * t319;
            let t468 = t465 * t76;
            let t470 = t97 * t298;
            let t474 = t321 * t76;
            let t476 = t83 * t298;
            let t478 = t321 * t121;
            let t484 = -f64x8::splat(1.38472194e-08) * t139 * t356 - f64x8::splat(3.76702959e-08) * t444 * t116 - f64x8::splat(0.00957417512) * t456 + f64x8::splat(8.50272392e-09) * t444 * t103 - f64x8::splat(2.7652468e-07) * t465 * t130 + f64x8::splat(0.00940675747) * t468 + f64x8::splat(0.00940675747) * t470 - f64x8::splat(6.91592964e-09) * t403 * t103 + f64x8::splat(0.100339208) * t474 + f64x8::splat(0.100339208) * t476 + f64x8::splat(0.00119130546) * t478 + f64x8::splat(0.0120828626792125) * t352 + f64x8::splat(0.01406365375513125) * t354 + f64x8::splat(0.00119130546) * t83 * t431;
            let t487 = t321 * t126;
            let t492 = t321 * t110;
            let t497 = t321 * t116;
            let t502 = t190 * t298;
            let t504 = t321 * t103;
            let t511 = -f64x8::splat(0.00303347141) * t487 + f64x8::splat(0.013938308465540625) * t373 - f64x8::splat(0.00303347141) * t83 * t383 - f64x8::splat(5.14204676e-05) * t492 - f64x8::splat(0.00048005288013375) * t350 - f64x8::splat(5.14204676e-05) * t83 * t356 + f64x8::splat(0.000822139896) * t497 - f64x8::splat(0.004729415517815625) * t371 + f64x8::splat(0.000822139896) * t83 * t393 - f64x8::splat(0.043464346) * t502 - f64x8::splat(9.40351563e-06) * t504 + f64x8::splat(0.000138149743606875) * t369 - f64x8::splat(9.40351563e-06) * t83 * t376 + f64x8::splat(0.00293253041) * t190 * t383;
            let t524 = t420 * t76;
            let t526 = t155 * t298;
            let t536 = t321 * t130;
            let t538 = -f64x8::splat(0.00035104103) * t190 * t393 + f64x8::splat(0.00182906057) * t190 * t431 + f64x8::splat(0.00339308972) * t172 * t431 - f64x8::splat(2.16860568e-08) * t364 * t110 - f64x8::splat(2.16860568e-08) * t172 * t356 - f64x8::splat(0.5522247359125) * t400 - f64x8::splat(0.18458962865625) * t322 + f64x8::splat(0.0162638575) * t524 + f64x8::splat(0.0162638575) * t526 + f64x8::splat(6.74910119e-09) * t364 * t103 + f64x8::splat(6.74910119e-09) * t172 * t376 + f64x8::splat(0.00631891628) * t420 * t126 + f64x8::splat(0.00631891628) * t155 * t383 - f64x8::splat(0.00879090772) * t536;
            let t551 = t444 * t76;
            let t553 = t139 * t298;
            let t567 = f64x8::splat(8.88525527e-09) * t97 * t376 + f64x8::splat(5.54588743e-08) * t465 * t121 - f64x8::splat(7.74224962e-09) * t465 * t110 - f64x8::splat(7.74224962e-09) * t97 * t356 + f64x8::splat(8.88525527e-09) * t465 * t103 - f64x8::splat(0.004373652639371875) * t298 - f64x8::splat(0.00884148272) * t551 - f64x8::splat(0.00884148272) * t553 - f64x8::splat(4.93824365e-09) * t420 * t103 - f64x8::splat(4.93824365e-09) * t155 * t376 - f64x8::splat(0.00896771404) * t444 * t126 - f64x8::splat(0.00896771404) * t139 * t383 - f64x8::splat(0.0188495102) * t444 * t130 - f64x8::splat(3.76702959e-08) * t139 * t393;
            let t575 = t403 * t76;
            let t595 = f64x8::splat(8.50272392e-09) * t139 * t376 - f64x8::splat(2.65114646e-08) * t226 * t383 + f64x8::splat(0.48014796319875) * t398 + f64x8::splat(1.69805915e-07) * t403 * t130 - f64x8::splat(0.00957417512) * t575 - f64x8::splat(4.16393106e-08) * t403 * t121 - f64x8::splat(4.16393106e-08) * t226 * t431 - f64x8::splat(0.0028938240791087965) * t319 + f64x8::splat(0.034725888949305554) * t303 - f64x8::splat(6.91592964e-09) * t226 * t376 + f64x8::splat(5.54588743e-08) * t97 * t431 + f64x8::splat(5.05920757e-08) * t465 * t126 + f64x8::splat(5.05920757e-08) * t97 * t383 - f64x8::splat(3.38128188e-08) * t465 * t116 - f64x8::splat(3.38128188e-08) * t97 * t393;
            let t598 = t349 + t410 + t451 + t484 + t511 + t538 + t567 + t595;
            let t603 = ((t3).select(f64x8::splat(0.0), -t7 * t256 * t250 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t598));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t603 + f64x8::splat(2.0) * t254;
            acc_vrho = tvrho0;
            let t606 = t26 * t34;
            let t607 = f64x8::splat(5.0) / f64x8::splat(72.0) * t606;
            let t608 = ((t54).select(-t607, f64x8::splat(0.0)));
            let t611 = t62 * t608;
            let t613 = t277 * t608;
            let t616 = ((t54).select(f64x8::splat(0.0), -t607));
            let t617 = t65 * t616;
            let t623 = f64x8::splat(3.0) * t289 * t616 + f64x8::splat(3.0) * t292 * t616;
            let t626 = ((t53).select(f64x8::splat(6.0) * t60 * t608 + f64x8::splat(3.0) * t611 - f64x8::splat(12.0) * t613, f64x8::splat(6.0) * t281 * t617 + t288 * t623));
            let t628 = t26 * t41;
            let t631 = t311 * v_rho;
            let t635 = t28 / t19 / t631 * t317;
            let t636 = t308 * v_sigma * t635;
            let t638 = t628 / f64x8::splat(12.0) - t636 / f64x8::splat(144.0);
            let t639 = t88 * t638;
            let t641 = t87 * t638;
            let t643 = t84 * t638;
            let t647 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t639 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t641 + f64x8::splat(945.0) / f64x8::splat(16.0) * t643 - f64x8::splat(35.0) / f64x8::splat(192.0) * t628 + f64x8::splat(35.0) / f64x8::splat(2304.0) * t636;
            let t650 = t77 * t626;
            let t653 = f64x8::splat(15.0) / f64x8::splat(2.0) * t650 - f64x8::splat(3.0) / f64x8::splat(2.0) * t626;
            let t658 = t79 * t626;
            let t662 = f64x8::splat(315.0) / f64x8::splat(8.0) * t658 - f64x8::splat(105.0) / f64x8::splat(4.0) * t650 + f64x8::splat(15.0) / f64x8::splat(8.0) * t626;
            let t675 = t83 * t638;
            let t684 = -f64x8::splat(0.004373652639371875) * t626 + f64x8::splat(5.05920757e-08) * t647 * t126 + f64x8::splat(5.05920757e-08) * t97 * t653 - f64x8::splat(3.38128188e-08) * t647 * t116 - f64x8::splat(3.38128188e-08) * t97 * t662 - f64x8::splat(7.74224962e-09) * t647 * t110 + f64x8::splat(5.09417745e-07) * t239 * t626 - f64x8::splat(0.0565485306) * t151 * t626 - f64x8::splat(0.0548733873) * t168 * t626 + f64x8::splat(0.0842036616) * t187 * t626 - f64x8::splat(6.69043971e-09) * t675 * t103 + f64x8::splat(2.006940657e-08) * t675 * t110 - f64x8::splat(0.00105312309) * t675 * t116 + f64x8::splat(0.00548718171) * t675 * t121;
            let t695 = t78 * t626;
            let t697 = t76 * t626;
            let t699 = f64x8::splat(35.0) / f64x8::splat(2.0) * t695 - f64x8::splat(15.0) / f64x8::splat(2.0) * t697;
            let t704 = t190 * t626;
            let t706 = t638 * t103;
            let t708 = t106 * t626;
            let t713 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t708 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t658 + f64x8::splat(945.0) / f64x8::splat(16.0) * t650 - f64x8::splat(35.0) / f64x8::splat(16.0) * t626;
            let t716 = t638 * t110;
            let t718 = t99 * t626;
            let t722 = f64x8::splat(693.0) / f64x8::splat(8.0) * t718 - f64x8::splat(315.0) / f64x8::splat(4.0) * t695 + f64x8::splat(105.0) / f64x8::splat(8.0) * t697;
            let t725 = t638 * t116;
            let t729 = f64x8::splat(0.00879759123) * t675 * t126 - f64x8::splat(0.0450310908) * t675 * t130 - f64x8::splat(0.0450310908) * t206 * t626 - f64x8::splat(0.130393038) * t675 * t76 - f64x8::splat(0.02637272316) * t221 * t626 + f64x8::splat(0.00182906057) * t190 * t699 + f64x8::splat(0.00293253041) * t190 * t653 - f64x8::splat(0.043464346) * t704 - f64x8::splat(9.40351563e-06) * t706 - f64x8::splat(9.40351563e-06) * t83 * t713 - f64x8::splat(5.14204676e-05) * t716 - f64x8::splat(5.14204676e-05) * t83 * t722 + f64x8::splat(0.000822139896) * t725 + f64x8::splat(0.000822139896) * t83 * t662;
            let t731 = t93 * t638;
            let t733 = t85 * t638;
            let t736 = f64x8::splat(693.0) / f64x8::splat(8.0) * t731 - f64x8::splat(315.0) / f64x8::splat(4.0) * t733 + f64x8::splat(105.0) / f64x8::splat(8.0) * t675;
            let t754 = f64x8::splat(315.0) / f64x8::splat(8.0) * t641 - f64x8::splat(105.0) / f64x8::splat(4.0) * t643 + f64x8::splat(5.0) / f64x8::splat(32.0) * t628 - f64x8::splat(5.0) / f64x8::splat(384.0) * t636;
            let t763 = t736 * t76;
            let t765 = -f64x8::splat(2.65114646e-08) * t736 * t126 - f64x8::splat(2.65114646e-08) * t226 * t653 + f64x8::splat(1.69805915e-07) * t736 * t130 - f64x8::splat(4.16393106e-08) * t736 * t121 - f64x8::splat(4.16393106e-08) * t226 * t699 + f64x8::splat(6.94482484e-09) * t736 * t110 + f64x8::splat(1.493833915228125) * t643 - f64x8::splat(1.38472194e-08) * t754 * t110 - f64x8::splat(0.00048005288013375) * t718 - f64x8::splat(1.38472194e-08) * t139 * t722 + f64x8::splat(0.48014796319875) * t731 - f64x8::splat(0.5522247359125) * t733 - f64x8::splat(0.18458962865625) * t675 - f64x8::splat(0.00957417512) * t763;
            let t766 = t226 * t626;
            let t792 = -f64x8::splat(0.00957417512) * t766 + f64x8::splat(8.50272392e-09) * t754 * t103 + f64x8::splat(0.000138149743606875) * t708 - f64x8::splat(0.004729415517815625) * t658 + f64x8::splat(8.50272392e-09) * t139 * t713 - f64x8::splat(7.74224962e-09) * t97 * t722 + f64x8::splat(8.88525527e-09) * t647 * t103 + f64x8::splat(8.88525527e-09) * t97 * t713 - f64x8::splat(3.76702959e-08) * t754 * t116 - f64x8::splat(3.76702959e-08) * t139 * t662 + f64x8::splat(1.62238741e-07) * t754 * t121 + f64x8::splat(6.94482484e-09) * t226 * t722 + f64x8::splat(2.36391411e-08) * t736 * t116 + f64x8::splat(2.36391411e-08) * t226 * t662;
            let t804 = f64x8::splat(15.0) / f64x8::splat(2.0) * t643 - t628 / f64x8::splat(8.0) + t636 / f64x8::splat(96.0);
            let t815 = t804 * t76;
            let t817 = t172 * t626;
            let t827 = -f64x8::splat(6.91592964e-09) * t736 * t103 - f64x8::splat(6.91592964e-09) * t226 * t713 + f64x8::splat(0.000896739466) * t172 * t662 + f64x8::splat(0.00339308972) * t804 * t121 + f64x8::splat(0.00339308972) * t172 * t699 - f64x8::splat(0.00845508103) * t804 * t126 - f64x8::splat(0.00845508103) * t172 * t653 + f64x8::splat(0.0280678872) * t804 * t130 - f64x8::splat(0.0182177954) * t815 - f64x8::splat(0.0182177954) * t817 - f64x8::splat(2.23014657e-09) * t190 * t713 + f64x8::splat(6.68980219e-09) * t190 * t722 - f64x8::splat(0.00035104103) * t190 * t662 + f64x8::splat(1.62238741e-07) * t139 * t699;
            let t834 = t754 * t76;
            let t836 = t139 * t626;
            let t840 = f64x8::splat(35.0) / f64x8::splat(2.0) * t733 - f64x8::splat(15.0) / f64x8::splat(2.0) * t675;
            let t846 = t647 * t76;
            let t848 = t97 * t626;
            let t858 = -f64x8::splat(0.00896771404) * t754 * t126 - f64x8::splat(0.00896771404) * t139 * t653 - f64x8::splat(0.0188495102) * t754 * t130 - f64x8::splat(0.00884148272) * t834 - f64x8::splat(0.00884148272) * t836 - f64x8::splat(4.93824365e-09) * t840 * t103 + f64x8::splat(0.742180708644375) * t639 - f64x8::splat(2.7652468e-07) * t647 * t130 + f64x8::splat(0.00940675747) * t846 + f64x8::splat(0.00940675747) * t848 + f64x8::splat(5.54588743e-08) * t647 * t121 + f64x8::splat(5.54588743e-08) * t97 * t699 - f64x8::splat(4.93824365e-09) * t155 * t713 + f64x8::splat(9.12223751e-09) * t840 * t110;
            let t872 = t638 * t121;
            let t878 = t638 * t126;
            let t883 = t638 * t130;
            let t885 = f64x8::splat(9.12223751e-09) * t155 * t722 + f64x8::splat(2.09603871e-08) * t840 * t116 + f64x8::splat(2.09603871e-08) * t155 * t662 - f64x8::splat(7.90811707e-08) * t840 * t121 - f64x8::splat(7.90811707e-08) * t155 * t699 + f64x8::splat(0.00631891628) * t840 * t126 + f64x8::splat(0.00119130546) * t872 + f64x8::splat(0.0120828626792125) * t695 + f64x8::splat(0.01406365375513125) * t697 + f64x8::splat(0.00119130546) * t83 * t699 - f64x8::splat(0.00303347141) * t878 + f64x8::splat(0.013938308465540625) * t650 - f64x8::splat(0.00303347141) * t83 * t653 - f64x8::splat(0.00879090772) * t883;
            let t886 = t638 * t76;
            let t888 = t83 * t626;
            let t899 = t840 * t76;
            let t901 = t155 * t626;
            let t913 = f64x8::splat(0.100339208) * t886 + f64x8::splat(0.100339208) * t888 - f64x8::splat(1.975305997940625) * t641 + f64x8::splat(0.0010851840296657986) * t636 - f64x8::splat(8.2957404e-07) * t133 * t626 - f64x8::splat(0.013022208355989584) * t628 + f64x8::splat(0.00631891628) * t155 * t653 - f64x8::splat(0.0182911291) * t840 * t130 + f64x8::splat(0.0162638575) * t899 + f64x8::splat(0.0162638575) * t901 + f64x8::splat(6.74910119e-09) * t804 * t103 + f64x8::splat(6.74910119e-09) * t172 * t713 - f64x8::splat(2.16860568e-08) * t804 * t110 - f64x8::splat(2.16860568e-08) * t172 * t722 + f64x8::splat(0.000896739466) * t804 * t116;
            let t916 = t684 + t729 + t765 + t792 + t827 + t858 + t885 + t913;
            let t920 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t916));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t920;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t924 = f64x8::splat(5.0) / f64x8::splat(9.0) * t29 * t46 * t26;
            let t925 = ((t54).select(t924, f64x8::splat(0.0)));
            let t928 = t62 * t925;
            let t930 = t277 * t925;
            let t933 = ((t54).select(f64x8::splat(0.0), t924));
            let t934 = t65 * t933;
            let t940 = f64x8::splat(3.0) * t289 * t933 + f64x8::splat(3.0) * t292 * t933;
            let t943 = ((t53).select(f64x8::splat(6.0) * t60 * t925 + f64x8::splat(3.0) * t928 - f64x8::splat(12.0) * t930, f64x8::splat(6.0) * t281 * t934 + t288 * t940));
            let t944 = t83 * t943;
            let t946 = t78 * t943;
            let t948 = t76 * t943;
            let t952 = f64x8::splat(35.0) / f64x8::splat(2.0) * t946 - f64x8::splat(15.0) / f64x8::splat(2.0) * t948;
            let t955 = t77 * t943;
            let t959 = f64x8::splat(15.0) / f64x8::splat(2.0) * t955 - f64x8::splat(3.0) / f64x8::splat(2.0) * t943;
            let t962 = t79 * t943;
            let t967 = f64x8::splat(315.0) / f64x8::splat(8.0) * t962 - f64x8::splat(105.0) / f64x8::splat(4.0) * t955 + f64x8::splat(15.0) / f64x8::splat(8.0) * t943;
            let t970 = t99 * t943;
            let t975 = f64x8::splat(693.0) / f64x8::splat(8.0) * t970 - f64x8::splat(315.0) / f64x8::splat(4.0) * t946 + f64x8::splat(105.0) / f64x8::splat(8.0) * t948;
            let t978 = t190 * t943;
            let t980 = t106 * t943;
            let t986 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t980 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t962 + f64x8::splat(945.0) / f64x8::splat(16.0) * t955 - f64x8::splat(35.0) / f64x8::splat(16.0) * t943;
            let t991 = f64x8::splat(0.100339208) * t944 + f64x8::splat(0.0120828626792125) * t946 + f64x8::splat(0.01406365375513125) * t948 + f64x8::splat(0.00119130546) * t83 * t952 + f64x8::splat(0.013938308465540625) * t955 - f64x8::splat(0.00303347141) * t83 * t959 - f64x8::splat(0.004729415517815625) * t962 + f64x8::splat(0.000822139896) * t83 * t967 - f64x8::splat(0.00048005288013375) * t970 - f64x8::splat(5.14204676e-05) * t83 * t975 - f64x8::splat(0.043464346) * t978 + f64x8::splat(0.000138149743606875) * t980 - f64x8::splat(9.40351563e-06) * t83 * t986 + f64x8::splat(0.00182906057) * t190 * t952;
            let t1002 = t172 * t943;
            let t1011 = t97 * t943;
            let t1019 = f64x8::splat(0.00293253041) * t190 * t959 + f64x8::splat(6.68980219e-09) * t190 * t975 - f64x8::splat(0.00035104103) * t190 * t967 - f64x8::splat(2.23014657e-09) * t190 * t986 - f64x8::splat(0.00845508103) * t172 * t959 - f64x8::splat(0.0182177954) * t1002 + f64x8::splat(0.000896739466) * t172 * t967 + f64x8::splat(0.00339308972) * t172 * t952 - f64x8::splat(2.16860568e-08) * t172 * t975 - f64x8::splat(0.004373652639371875) * t943 + f64x8::splat(0.00940675747) * t1011 - f64x8::splat(6.91592964e-09) * t226 * t986 + f64x8::splat(5.05920757e-08) * t97 * t959 - f64x8::splat(3.38128188e-08) * t97 * t967;
            let t1027 = t155 * t943;
            let t1043 = t139 * t943;
            let t1049 = f64x8::splat(5.54588743e-08) * t97 * t952 - f64x8::splat(7.74224962e-09) * t97 * t975 + f64x8::splat(8.88525527e-09) * t97 * t986 + f64x8::splat(0.0162638575) * t1027 + f64x8::splat(6.74910119e-09) * t172 * t986 - f64x8::splat(7.90811707e-08) * t155 * t952 + f64x8::splat(0.00631891628) * t155 * t959 + f64x8::splat(9.12223751e-09) * t155 * t975 + f64x8::splat(2.09603871e-08) * t155 * t967 - f64x8::splat(4.93824365e-09) * t155 * t986 - f64x8::splat(0.00896771404) * t139 * t959 - f64x8::splat(0.00884148272) * t1043 - f64x8::splat(3.76702959e-08) * t139 * t967 + f64x8::splat(1.62238741e-07) * t139 * t952;
            let t1052 = t226 * t943;
            let t1078 = -f64x8::splat(1.38472194e-08) * t139 * t975 - f64x8::splat(0.00957417512) * t1052 + f64x8::splat(8.50272392e-09) * t139 * t986 - f64x8::splat(4.16393106e-08) * t226 * t952 - f64x8::splat(2.65114646e-08) * t226 * t959 + f64x8::splat(6.94482484e-09) * t226 * t975 + f64x8::splat(2.36391411e-08) * t226 * t967 - f64x8::splat(0.02637272316) * t221 * t943 - f64x8::splat(0.0450310908) * t206 * t943 - f64x8::splat(0.0548733873) * t168 * t943 + f64x8::splat(0.0842036616) * t187 * t943 - f64x8::splat(0.0565485306) * t151 * t943 + f64x8::splat(5.09417745e-07) * t239 * t943 - f64x8::splat(8.2957404e-07) * t133 * t943;
            let t1080 = t991 + t1019 + t1049 + t1078;
            let t1084 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1080));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t1084;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
