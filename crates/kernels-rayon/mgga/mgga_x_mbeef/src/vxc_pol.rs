//! MGGA_X_MBEEF vxc pol kernel — explicit SIMD (bit-exact).
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeef_vxc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = t19 + f64x8::splat(1.0);
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = v_sigma0 * t39;
            let t43 = f64x8::splat(6.5124) + t34 * t40 / f64x8::splat(24.0);
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t34 * t40 * t44;
            let t49 = t46 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t50 = t49 * t49;
            let t52 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t50;
            let t54 = f64x8::splat(1.0) / t37 / v_rho0;
            let t60 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau0 * t54 - t40 / f64x8::splat(8.0)) * t29 * t33;
            let t61 = (f64x8::splat(10000.0)).simd_le(t60);
            let t62 = (f64x8::splat(10000.0)).simd_lt(t60);
            let t63 = ((t62).select(t60, f64x8::splat(10000.0)));
            let t64 = t63 * t63;
            let t67 = t64 * t63;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t64 * t64;
            let t70 = f64x8::splat(1.0) / t69;
            let t73 = ((t62).select(f64x8::splat(10000.0), t60));
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) - t74;
            let t76 = t75 * t75;
            let t77 = t76 * t75;
            let t78 = t74 * t73;
            let t79 = f64x8::splat(1.0) + t78;
            let t81 = t78 * t79 + f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t84 = ((t61).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t64 - t68 + f64x8::splat(3.0) * t70, -t77 * t82));
            let t85 = t84 * t84;
            let t86 = t85 * t84;
            let t87 = t85 * t85;
            let t88 = t87 * t86;
            let t90 = t87 * t84;
            let t94 = f64x8::splat(429.0) / f64x8::splat(16.0) * t88 - f64x8::splat(693.0) / f64x8::splat(16.0) * t90 + f64x8::splat(315.0) / f64x8::splat(16.0) * t86 - f64x8::splat(35.0) / f64x8::splat(16.0) * t84;
            let t97 = t87 * t85;
            let t101 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t97 - f64x8::splat(315.0) / f64x8::splat(16.0) * t87 + f64x8::splat(105.0) / f64x8::splat(16.0) * t85;
            let t107 = f64x8::splat(63.0) / f64x8::splat(8.0) * t90 - f64x8::splat(35.0) / f64x8::splat(4.0) * t86 + f64x8::splat(15.0) / f64x8::splat(8.0) * t84;
            let t112 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t87 - f64x8::splat(15.0) / f64x8::splat(4.0) * t85;
            let t117 = f64x8::splat(5.0) / f64x8::splat(2.0) * t86 - f64x8::splat(3.0) / f64x8::splat(2.0) * t84;
            let t121 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t85;
            let t124 = t52 * t84;
            let t138 = t49 * t84;
            let t140 = t50 * t50;
            let t141 = t140 * t49;
            let t143 = t50 * t49;
            let t146 = f64x8::splat(63.0) / f64x8::splat(8.0) * t141 - f64x8::splat(35.0) / f64x8::splat(4.0) * t143 + f64x8::splat(5.0) / f64x8::splat(32.0) * t46 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t149 = -f64x8::splat(0.013022208355989584) * t46 - f64x8::splat(2.23014657e-09) * t52 * t94 + f64x8::splat(6.68980219e-09) * t52 * t101 - f64x8::splat(0.00035104103) * t52 * t107 + f64x8::splat(0.00182906057) * t52 * t112 + f64x8::splat(0.00293253041) * t52 * t117 - f64x8::splat(0.0150103636) * t52 * t121 - f64x8::splat(0.043464346) * t124 - f64x8::splat(9.40351563e-06) * t49 * t94 - f64x8::splat(5.14204676e-05) * t49 * t101 + f64x8::splat(0.000822139896) * t49 * t107 + f64x8::splat(0.00119130546) * t49 * t112 - f64x8::splat(0.00303347141) * t49 * t117 - f64x8::splat(0.00879090772) * t49 * t121 + f64x8::splat(0.100339208) * t138 + f64x8::splat(8.50272392e-09) * t146 * t94;
            let t160 = t146 * t84;
            let t164 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t140 - f64x8::splat(15.0) / f64x8::splat(4.0) * t50;
            let t177 = t164 * t84;
            let t181 = f64x8::splat(5.0) / f64x8::splat(2.0) * t143 - t46 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t188 = -f64x8::splat(1.38472194e-08) * t146 * t101 - f64x8::splat(3.76702959e-08) * t146 * t107 + f64x8::splat(1.62238741e-07) * t146 * t112 - f64x8::splat(0.00896771404) * t146 * t117 - f64x8::splat(0.0188495102) * t146 * t121 - f64x8::splat(0.00884148272) * t160 - f64x8::splat(4.93824365e-09) * t164 * t94 + f64x8::splat(9.12223751e-09) * t164 * t101 + f64x8::splat(2.09603871e-08) * t164 * t107 - f64x8::splat(7.90811707e-08) * t164 * t112 + f64x8::splat(0.00631891628) * t164 * t117 - f64x8::splat(0.0182911291) * t164 * t121 + f64x8::splat(0.0162638575) * t177 + f64x8::splat(6.74910119e-09) * t181 * t94 - f64x8::splat(2.16860568e-08) * t181 * t101 + f64x8::splat(0.000896739466) * t181 * t107;
            let t196 = t181 * t84;
            let t198 = t140 * t143;
            let t203 = f64x8::splat(429.0) / f64x8::splat(16.0) * t198 - f64x8::splat(693.0) / f64x8::splat(16.0) * t141 + f64x8::splat(315.0) / f64x8::splat(16.0) * t143 - f64x8::splat(35.0) / f64x8::splat(192.0) * t46 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t204 = t203 * t84;
            let t206 = t140 * t50;
            let t210 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t206 - f64x8::splat(315.0) / f64x8::splat(16.0) * t140 + f64x8::splat(105.0) / f64x8::splat(16.0) * t50;
            let t223 = t210 * t84;
            let t233 = f64x8::splat(0.00339308972) * t181 * t112 - f64x8::splat(0.00845508103) * t181 * t117 + f64x8::splat(0.0280678872) * t181 * t121 - f64x8::splat(0.0182177954) * t196 + f64x8::splat(0.00940675747) * t204 - f64x8::splat(6.91592964e-09) * t210 * t94 + f64x8::splat(6.94482484e-09) * t210 * t101 + f64x8::splat(2.36391411e-08) * t210 * t107 - f64x8::splat(4.16393106e-08) * t210 * t112 - f64x8::splat(2.65114646e-08) * t210 * t117 + f64x8::splat(1.69805915e-07) * t210 * t121 - f64x8::splat(0.00957417512) * t223 + f64x8::splat(8.88525527e-09) * t203 * t94 - f64x8::splat(7.74224962e-09) * t203 * t101 - f64x8::splat(3.38128188e-08) * t203 * t107 + f64x8::splat(5.54588743e-08) * t203 * t112;
            let t251 = f64x8::splat(1.3805672252189969) + f64x8::splat(5.05920757e-08) * t203 * t117 - f64x8::splat(2.7652468e-07) * t203 * t121 + f64x8::splat(0.106025815520625) * t198 - f64x8::splat(0.395061199588125) * t141 + f64x8::splat(0.497944638409375) * t143 + f64x8::splat(1.9735677658125e-05) * t88 - f64x8::splat(0.004373652639371875) * t84 - f64x8::splat(0.000945883103563125) * t90 + f64x8::splat(0.004646102821846875) * t86 - f64x8::splat(8.0008813355625e-05) * t97 + f64x8::splat(0.003020715669803125) * t87 + f64x8::splat(0.007031826877565625) * t85 + f64x8::splat(0.080024660533125) * t206 - f64x8::splat(0.138056183978125) * t140 - f64x8::splat(0.092294814328125) * t50;
            let t253 = t149 + t188 + t233 + t251;
            let t257 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t253));
            let t258 = (v_rho1).simd_le(dens_threshold);
            let t259 = -t17;
            let t261 = ((t15).select(t12, (t11).select(t16, t259 * t8)));
            let t262 = t261 + f64x8::splat(1.0);
            let t263 = (t262).simd_le(zeta_threshold);
            let t264 = (simd::cbrt(t262));
            let t266 = ((t263).select(t23, t264 * t262));
            let t267 = t266 * t27;
            let t268 = v_rho1 * v_rho1;
            let t269 = (simd::cbrt(v_rho1));
            let t270 = t269 * t269;
            let t272 = f64x8::splat(1.0) / t270 / t268;
            let t273 = v_sigma2 * t272;
            let t276 = f64x8::splat(6.5124) + t34 * t273 / f64x8::splat(24.0);
            let t277 = f64x8::splat(1.0) / t276;
            let t279 = t34 * t273 * t277;
            let t281 = t279 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t282 = t281 * t281;
            let t283 = t282 * t282;
            let t284 = t283 * t281;
            let t286 = t282 * t281;
            let t289 = f64x8::splat(63.0) / f64x8::splat(8.0) * t284 - f64x8::splat(35.0) / f64x8::splat(4.0) * t286 + f64x8::splat(5.0) / f64x8::splat(32.0) * t279 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t291 = f64x8::splat(1.0) / t270 / v_rho1;
            let t297 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t291 - t273 / f64x8::splat(8.0)) * t29 * t33;
            let t298 = (f64x8::splat(10000.0)).simd_le(t297);
            let t299 = (f64x8::splat(10000.0)).simd_lt(t297);
            let t300 = ((t299).select(t297, f64x8::splat(10000.0)));
            let t301 = t300 * t300;
            let t304 = t301 * t300;
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t301 * t301;
            let t307 = f64x8::splat(1.0) / t306;
            let t310 = ((t299).select(f64x8::splat(10000.0), t297));
            let t311 = t310 * t310;
            let t312 = f64x8::splat(1.0) - t311;
            let t313 = t312 * t312;
            let t314 = t313 * t312;
            let t315 = t311 * t310;
            let t316 = f64x8::splat(1.0) + t315;
            let t318 = t315 * t316 + f64x8::splat(1.0);
            let t319 = f64x8::splat(1.0) / t318;
            let t321 = ((t298).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t301 - t305 + f64x8::splat(3.0) * t307, -t314 * t319));
            let t322 = t289 * t321;
            let t326 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t283 - f64x8::splat(15.0) / f64x8::splat(4.0) * t282;
            let t327 = t321 * t321;
            let t328 = t327 * t321;
            let t329 = t327 * t327;
            let t330 = t329 * t328;
            let t332 = t329 * t321;
            let t336 = f64x8::splat(429.0) / f64x8::splat(16.0) * t330 - f64x8::splat(693.0) / f64x8::splat(16.0) * t332 + f64x8::splat(315.0) / f64x8::splat(16.0) * t328 - f64x8::splat(35.0) / f64x8::splat(16.0) * t321;
            let t339 = t329 * t327;
            let t343 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t339 - f64x8::splat(315.0) / f64x8::splat(16.0) * t329 + f64x8::splat(105.0) / f64x8::splat(16.0) * t327;
            let t349 = f64x8::splat(63.0) / f64x8::splat(8.0) * t332 - f64x8::splat(35.0) / f64x8::splat(4.0) * t328 + f64x8::splat(15.0) / f64x8::splat(8.0) * t321;
            let t354 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t329 - f64x8::splat(15.0) / f64x8::splat(4.0) * t327;
            let t359 = f64x8::splat(5.0) / f64x8::splat(2.0) * t328 - f64x8::splat(3.0) / f64x8::splat(2.0) * t321;
            let t363 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t327;
            let t366 = t326 * t321;
            let t370 = f64x8::splat(5.0) / f64x8::splat(2.0) * t286 - t279 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t379 = t283 * t286;
            let t384 = f64x8::splat(429.0) / f64x8::splat(16.0) * t379 - f64x8::splat(693.0) / f64x8::splat(16.0) * t284 + f64x8::splat(315.0) / f64x8::splat(16.0) * t286 - f64x8::splat(35.0) / f64x8::splat(192.0) * t279 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t391 = t384 * t321;
            let t393 = -f64x8::splat(0.00884148272) * t322 - f64x8::splat(4.93824365e-09) * t326 * t336 + f64x8::splat(9.12223751e-09) * t326 * t343 + f64x8::splat(2.09603871e-08) * t326 * t349 - f64x8::splat(7.90811707e-08) * t326 * t354 + f64x8::splat(0.00631891628) * t326 * t359 - f64x8::splat(0.0182911291) * t326 * t363 + f64x8::splat(0.0162638575) * t366 + f64x8::splat(6.74910119e-09) * t370 * t336 - f64x8::splat(2.16860568e-08) * t370 * t343 + f64x8::splat(0.000896739466) * t370 * t349 + f64x8::splat(0.00339308972) * t370 * t354 + f64x8::splat(5.54588743e-08) * t384 * t354 + f64x8::splat(5.05920757e-08) * t384 * t359 - f64x8::splat(2.7652468e-07) * t384 * t363 + f64x8::splat(0.00940675747) * t391;
            let t394 = t283 * t282;
            let t398 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t394 - f64x8::splat(315.0) / f64x8::splat(16.0) * t283 + f64x8::splat(105.0) / f64x8::splat(16.0) * t282;
            let t412 = t398 * t321;
            let t430 = -f64x8::splat(6.91592964e-09) * t398 * t336 + f64x8::splat(6.94482484e-09) * t398 * t343 + f64x8::splat(2.36391411e-08) * t398 * t349 - f64x8::splat(4.16393106e-08) * t398 * t354 - f64x8::splat(0.013022208355989584) * t279 - f64x8::splat(2.65114646e-08) * t398 * t359 + f64x8::splat(1.69805915e-07) * t398 * t363 - f64x8::splat(0.00957417512) * t412 + f64x8::splat(8.50272392e-09) * t289 * t336 - f64x8::splat(1.38472194e-08) * t289 * t343 - f64x8::splat(3.76702959e-08) * t289 * t349 + f64x8::splat(1.62238741e-07) * t289 * t354 - f64x8::splat(0.00896771404) * t289 * t359 - f64x8::splat(0.0188495102) * t289 * t363 + f64x8::splat(8.88525527e-09) * t384 * t336 - f64x8::splat(7.74224962e-09) * t384 * t343;
            let t438 = t370 * t321;
            let t441 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t282;
            let t454 = t441 * t321;
            let t456 = t281 * t321;
            let t466 = -f64x8::splat(3.38128188e-08) * t384 * t349 - f64x8::splat(0.00845508103) * t370 * t359 + f64x8::splat(0.0280678872) * t370 * t363 - f64x8::splat(0.0182177954) * t438 - f64x8::splat(2.23014657e-09) * t441 * t336 + f64x8::splat(6.68980219e-09) * t441 * t343 - f64x8::splat(0.00035104103) * t441 * t349 + f64x8::splat(0.00182906057) * t441 * t354 + f64x8::splat(0.00293253041) * t441 * t359 - f64x8::splat(0.0150103636) * t441 * t363 - f64x8::splat(0.043464346) * t454 + f64x8::splat(0.100339208) * t456 - f64x8::splat(0.00879090772) * t281 * t363 - f64x8::splat(0.00303347141) * t281 * t359 + f64x8::splat(0.00119130546) * t281 * t354 + f64x8::splat(0.000822139896) * t281 * t349;
            let t484 = f64x8::splat(1.3805672252189969) - f64x8::splat(8.0008813355625e-05) * t339 + f64x8::splat(0.003020715669803125) * t329 + f64x8::splat(0.007031826877565625) * t327 - f64x8::splat(0.092294814328125) * t282 - f64x8::splat(0.004373652639371875) * t321 - f64x8::splat(0.000945883103563125) * t332 + f64x8::splat(0.004646102821846875) * t328 + f64x8::splat(1.9735677658125e-05) * t330 + f64x8::splat(0.497944638409375) * t286 - f64x8::splat(0.138056183978125) * t283 - f64x8::splat(0.395061199588125) * t284 + f64x8::splat(0.080024660533125) * t394 + f64x8::splat(0.106025815520625) * t379 - f64x8::splat(5.14204676e-05) * t281 * t343 - f64x8::splat(9.40351563e-06) * t281 * t336;
            let t486 = t393 + t430 + t466 + t484;
            let t490 = ((t258).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t267 * t486));
            let tzk0 = t257 + t490;
            acc_zk = tzk0;
            let t491 = t7 * t7;
            let t492 = f64x8::splat(1.0) / t491;
            let t493 = t17 * t492;
            let t495 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t493)));
            let t498 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t495));
            let t499 = t498 * t27;
            let t503 = t27 * t27;
            let t504 = f64x8::splat(1.0) / t503;
            let t505 = t26 * t504;
            let t508 = t6 * t505 * t253 / f64x8::splat(8.0);
            let t511 = t35 * v_rho0;
            let t513 = f64x8::splat(1.0) / t37 / t511;
            let t514 = v_sigma0 * t513;
            let t519 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t39 + t514 / f64x8::splat(3.0)) * t29 * t33;
            let t520 = ((t62).select(t519, f64x8::splat(0.0)));
            let t523 = t70 * t520;
            let t526 = f64x8::splat(1.0) / t69 / t63;
            let t527 = t526 * t520;
            let t530 = t76 * t82;
            let t531 = ((t62).select(f64x8::splat(0.0), t519));
            let t532 = t73 * t531;
            let t535 = t81 * t81;
            let t536 = f64x8::splat(1.0) / t535;
            let t537 = t77 * t536;
            let t538 = t74 * t79;
            let t540 = t74 * t74;
            let t541 = t540 * t73;
            let t544 = f64x8::splat(3.0) * t538 * t531 + f64x8::splat(3.0) * t541 * t531;
            let t547 = ((t61).select(f64x8::splat(6.0) * t68 * t520 + f64x8::splat(3.0) * t523 - f64x8::splat(12.0) * t527, f64x8::splat(6.0) * t530 * t532 + t537 * t544));
            let t550 = t34 * t514 * t44;
            let t552 = t29 * t29;
            let t554 = f64x8::splat(1.0) / t31 / t30;
            let t555 = t552 * t554;
            let t556 = v_sigma0 * v_sigma0;
            let t557 = t35 * t35;
            let t558 = t557 * t35;
            let t560 = f64x8::splat(1.0) / t36 / t558;
            let t562 = t43 * t43;
            let t563 = f64x8::splat(1.0) / t562;
            let t565 = t555 * t556 * t560 * t563;
            let t571 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t550 + t565 / f64x8::splat(108.0);
            let t572 = t49 * t571;
            let t593 = -f64x8::splat(0.004373652639371875) * t547 + f64x8::splat(0.034725888949305554) * t550 - f64x8::splat(0.0014469120395543982) * t565 - f64x8::splat(0.02637272316) * t138 * t547 - f64x8::splat(0.00105312309) * t572 * t107 + f64x8::splat(0.00548718171) * t572 * t112 + f64x8::splat(0.00879759123) * t572 * t117 - f64x8::splat(0.0450310908) * t572 * t121 - f64x8::splat(0.0450310908) * t124 * t547 - f64x8::splat(0.130393038) * t572 * t84 + f64x8::splat(0.0842036616) * t196 * t547 - f64x8::splat(6.69043971e-09) * t572 * t94 + f64x8::splat(2.006940657e-08) * t572 * t101 - f64x8::splat(0.0548733873) * t177 * t547;
            let t600 = t571 * t121;
            let t602 = t571 * t84;
            let t604 = t49 * t547;
            let t606 = t571 * t112;
            let t608 = t86 * t547;
            let t610 = t84 * t547;
            let t614 = f64x8::splat(35.0) / f64x8::splat(2.0) * t608 - f64x8::splat(15.0) / f64x8::splat(2.0) * t610;
            let t617 = t571 * t117;
            let t619 = t85 * t547;
            let t623 = f64x8::splat(15.0) / f64x8::splat(2.0) * t619 - f64x8::splat(3.0) / f64x8::splat(2.0) * t547;
            let t626 = t571 * t101;
            let t628 = -f64x8::splat(0.0565485306) * t160 * t547 + f64x8::splat(5.09417745e-07) * t223 * t547 - f64x8::splat(8.2957404e-07) * t204 * t547 - f64x8::splat(0.00879090772) * t600 + f64x8::splat(0.100339208) * t602 + f64x8::splat(0.100339208) * t604 + f64x8::splat(0.00119130546) * t606 + f64x8::splat(0.0120828626792125) * t608 + f64x8::splat(0.01406365375513125) * t610 + f64x8::splat(0.00119130546) * t49 * t614 - f64x8::splat(0.00303347141) * t617 + f64x8::splat(0.013938308465540625) * t619 - f64x8::splat(0.00303347141) * t49 * t623 - f64x8::splat(5.14204676e-05) * t626;
            let t630 = t90 * t547;
            let t635 = f64x8::splat(693.0) / f64x8::splat(8.0) * t630 - f64x8::splat(315.0) / f64x8::splat(4.0) * t608 + f64x8::splat(105.0) / f64x8::splat(8.0) * t610;
            let t638 = t571 * t107;
            let t640 = t87 * t547;
            let t645 = f64x8::splat(315.0) / f64x8::splat(8.0) * t640 - f64x8::splat(105.0) / f64x8::splat(4.0) * t619 + f64x8::splat(15.0) / f64x8::splat(8.0) * t547;
            let t648 = t571 * t94;
            let t650 = t97 * t547;
            let t656 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t650 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t640 + f64x8::splat(945.0) / f64x8::splat(16.0) * t619 - f64x8::splat(35.0) / f64x8::splat(16.0) * t547;
            let t661 = t52 * t547;
            let t669 = t50 * t571;
            let t671 = -f64x8::splat(0.00048005288013375) * t630 - f64x8::splat(5.14204676e-05) * t49 * t635 + f64x8::splat(0.000822139896) * t638 - f64x8::splat(0.004729415517815625) * t640 + f64x8::splat(0.000822139896) * t49 * t645 - f64x8::splat(9.40351563e-06) * t648 + f64x8::splat(0.000138149743606875) * t650 - f64x8::splat(9.40351563e-06) * t49 * t656 + f64x8::splat(0.00293253041) * t52 * t623 - f64x8::splat(0.043464346) * t661 + f64x8::splat(0.00182906057) * t52 * t614 + f64x8::splat(6.68980219e-09) * t52 * t635 - f64x8::splat(0.00035104103) * t52 * t645 + f64x8::splat(1.493833915228125) * t669;
            let t675 = f64x8::splat(15.0) / f64x8::splat(2.0) * t669 + t550 / f64x8::splat(3.0) - t565 / f64x8::splat(72.0);
            let t676 = t675 * t84;
            let t678 = t181 * t547;
            let t700 = t143 * t571;
            let t703 = -f64x8::splat(0.0182177954) * t676 - f64x8::splat(0.0182177954) * t678 - f64x8::splat(2.23014657e-09) * t52 * t656 - f64x8::splat(0.00845508103) * t675 * t117 - f64x8::splat(0.00845508103) * t181 * t623 + f64x8::splat(0.0280678872) * t675 * t121 + f64x8::splat(0.000896739466) * t675 * t107 + f64x8::splat(0.000896739466) * t181 * t645 + f64x8::splat(0.00339308972) * t675 * t112 + f64x8::splat(0.00339308972) * t181 * t614 - f64x8::splat(2.16860568e-08) * t675 * t101 - f64x8::splat(2.16860568e-08) * t181 * t635 - f64x8::splat(0.5522247359125) * t700 - f64x8::splat(0.18458962865625) * t572;
            let t708 = f64x8::splat(35.0) / f64x8::splat(2.0) * t700 - f64x8::splat(15.0) / f64x8::splat(2.0) * t572;
            let t709 = t708 * t84;
            let t711 = t164 * t547;
            let t735 = t140 * t571;
            let t737 = f64x8::splat(0.0162638575) * t709 + f64x8::splat(0.0162638575) * t711 + f64x8::splat(6.74910119e-09) * t675 * t94 + f64x8::splat(6.74910119e-09) * t181 * t656 + f64x8::splat(0.00631891628) * t708 * t117 + f64x8::splat(0.00631891628) * t164 * t623 - f64x8::splat(0.0182911291) * t708 * t121 + f64x8::splat(2.09603871e-08) * t708 * t107 + f64x8::splat(2.09603871e-08) * t164 * t645 - f64x8::splat(7.90811707e-08) * t708 * t112 - f64x8::splat(7.90811707e-08) * t164 * t614 + f64x8::splat(9.12223751e-09) * t708 * t101 + f64x8::splat(9.12223751e-09) * t164 * t635 - f64x8::splat(1.975305997940625) * t735;
            let t742 = f64x8::splat(315.0) / f64x8::splat(8.0) * t735 - f64x8::splat(105.0) / f64x8::splat(4.0) * t669 - f64x8::splat(5.0) / f64x8::splat(12.0) * t550 + f64x8::splat(5.0) / f64x8::splat(288.0) * t565;
            let t743 = t742 * t84;
            let t745 = t146 * t547;
            let t771 = -f64x8::splat(0.00884148272) * t743 - f64x8::splat(0.00884148272) * t745 - f64x8::splat(4.93824365e-09) * t708 * t94 - f64x8::splat(4.93824365e-09) * t164 * t656 - f64x8::splat(0.00896771404) * t742 * t117 - f64x8::splat(0.00896771404) * t146 * t623 - f64x8::splat(0.0188495102) * t742 * t121 - f64x8::splat(3.76702959e-08) * t146 * t645 + f64x8::splat(1.62238741e-07) * t742 * t112 + f64x8::splat(1.62238741e-07) * t146 * t614 - f64x8::splat(1.38472194e-08) * t742 * t101 - f64x8::splat(1.38472194e-08) * t146 * t635 - f64x8::splat(3.76702959e-08) * t742 * t107 + f64x8::splat(8.50272392e-09) * t742 * t94;
            let t777 = t141 * t571;
            let t782 = f64x8::splat(693.0) / f64x8::splat(8.0) * t777 - f64x8::splat(315.0) / f64x8::splat(4.0) * t700 + f64x8::splat(105.0) / f64x8::splat(8.0) * t572;
            let t785 = t782 * t84;
            let t787 = t210 * t547;
            let t805 = f64x8::splat(8.50272392e-09) * t146 * t656 - f64x8::splat(2.65114646e-08) * t210 * t623 + f64x8::splat(0.48014796319875) * t777 + f64x8::splat(1.69805915e-07) * t782 * t121 - f64x8::splat(0.00957417512) * t785 - f64x8::splat(0.00957417512) * t787 - f64x8::splat(4.16393106e-08) * t782 * t112 - f64x8::splat(4.16393106e-08) * t210 * t614 - f64x8::splat(2.65114646e-08) * t782 * t117 + f64x8::splat(2.36391411e-08) * t782 * t107 + f64x8::splat(2.36391411e-08) * t210 * t645 - f64x8::splat(6.91592964e-09) * t210 * t656 + f64x8::splat(6.94482484e-09) * t782 * t101 + f64x8::splat(6.94482484e-09) * t210 * t635;
            let t806 = t206 * t571;
            let t813 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t806 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t735 + f64x8::splat(945.0) / f64x8::splat(16.0) * t669 + f64x8::splat(35.0) / f64x8::splat(72.0) * t550 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t565;
            let t816 = t813 * t84;
            let t818 = t203 * t547;
            let t842 = f64x8::splat(0.742180708644375) * t806 - f64x8::splat(2.7652468e-07) * t813 * t121 + f64x8::splat(0.00940675747) * t816 + f64x8::splat(0.00940675747) * t818 - f64x8::splat(6.91592964e-09) * t782 * t94 + f64x8::splat(5.54588743e-08) * t203 * t614 + f64x8::splat(5.05920757e-08) * t813 * t117 + f64x8::splat(5.05920757e-08) * t203 * t623 - f64x8::splat(3.38128188e-08) * t813 * t107 - f64x8::splat(3.38128188e-08) * t203 * t645 + f64x8::splat(5.54588743e-08) * t813 * t112 - f64x8::splat(7.74224962e-09) * t813 * t101 - f64x8::splat(7.74224962e-09) * t203 * t635 + f64x8::splat(8.88525527e-09) * t813 * t94 + f64x8::splat(8.88525527e-09) * t203 * t656;
            let t845 = t593 + t628 + t671 + t703 + t737 + t771 + t805 + t842;
            let t850 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t499 * t253 - t508 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t845));
            let t851 = t259 * t492;
            let t853 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t851)));
            let t856 = ((t263).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t264 * t853));
            let t857 = t856 * t27;
            let t861 = t266 * t504;
            let t864 = t6 * t861 * t486 / f64x8::splat(8.0);
            let t866 = ((t258).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t857 * t486 - t864));
            let tvrho0 = t257 + t490 + t7 * (t850 + t866);
            acc_vrho_0 = tvrho0;
            let t870 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t493)));
            let t873 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t870));
            let t874 = t873 * t27;
            let t879 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t874 * t253 - t508));
            let t881 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t851)));
            let t884 = ((t263).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t264 * t881));
            let t885 = t884 * t27;
            let t891 = t268 * v_rho1;
            let t893 = f64x8::splat(1.0) / t270 / t891;
            let t894 = v_sigma2 * t893;
            let t899 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t272 + t894 / f64x8::splat(3.0)) * t29 * t33;
            let t900 = ((t299).select(t899, f64x8::splat(0.0)));
            let t903 = t307 * t900;
            let t906 = f64x8::splat(1.0) / t306 / t300;
            let t907 = t906 * t900;
            let t910 = t313 * t319;
            let t911 = ((t299).select(f64x8::splat(0.0), t899));
            let t912 = t310 * t911;
            let t915 = t318 * t318;
            let t916 = f64x8::splat(1.0) / t915;
            let t917 = t314 * t916;
            let t918 = t311 * t316;
            let t920 = t311 * t311;
            let t921 = t920 * t310;
            let t924 = f64x8::splat(3.0) * t918 * t911 + f64x8::splat(3.0) * t921 * t911;
            let t927 = ((t298).select(f64x8::splat(6.0) * t305 * t900 + f64x8::splat(3.0) * t903 - f64x8::splat(12.0) * t907, f64x8::splat(6.0) * t910 * t912 + t917 * t924));
            let t930 = t34 * t894 * t277;
            let t932 = v_sigma2 * v_sigma2;
            let t933 = t268 * t268;
            let t934 = t933 * t268;
            let t936 = f64x8::splat(1.0) / t269 / t934;
            let t938 = t276 * t276;
            let t939 = f64x8::splat(1.0) / t938;
            let t941 = t555 * t932 * t936 * t939;
            let t943 = t329 * t927;
            let t945 = t327 * t927;
            let t948 = f64x8::splat(315.0) / f64x8::splat(8.0) * t943 - f64x8::splat(105.0) / f64x8::splat(4.0) * t945 + f64x8::splat(15.0) / f64x8::splat(8.0) * t927;
            let t953 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t930 + t941 / f64x8::splat(108.0);
            let t954 = t394 * t953;
            let t956 = t283 * t953;
            let t958 = t282 * t953;
            let t962 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t954 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t956 + f64x8::splat(945.0) / f64x8::splat(16.0) * t958 + f64x8::splat(35.0) / f64x8::splat(72.0) * t930 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t941;
            let t965 = t328 * t927;
            let t967 = t321 * t927;
            let t969 = f64x8::splat(35.0) / f64x8::splat(2.0) * t965 - f64x8::splat(15.0) / f64x8::splat(2.0) * t967;
            let t976 = f64x8::splat(15.0) / f64x8::splat(2.0) * t945 - f64x8::splat(3.0) / f64x8::splat(2.0) * t927;
            let t981 = t332 * t927;
            let t985 = f64x8::splat(693.0) / f64x8::splat(8.0) * t981 - f64x8::splat(315.0) / f64x8::splat(4.0) * t965 + f64x8::splat(105.0) / f64x8::splat(8.0) * t967;
            let t990 = t384 * t927;
            let t994 = t339 * t927;
            let t999 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t994 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t943 + f64x8::splat(945.0) / f64x8::splat(16.0) * t945 - f64x8::splat(35.0) / f64x8::splat(16.0) * t927;
            let t1002 = -f64x8::splat(0.004373652639371875) * t927 + f64x8::splat(0.034725888949305554) * t930 - f64x8::splat(0.0014469120395543982) * t941 - f64x8::splat(3.38128188e-08) * t384 * t948 + f64x8::splat(5.54588743e-08) * t962 * t354 + f64x8::splat(5.54588743e-08) * t384 * t969 + f64x8::splat(5.05920757e-08) * t962 * t359 + f64x8::splat(5.05920757e-08) * t384 * t976 - f64x8::splat(7.74224962e-09) * t962 * t343 - f64x8::splat(7.74224962e-09) * t384 * t985 - f64x8::splat(3.38128188e-08) * t962 * t349 + f64x8::splat(0.00940675747) * t990 + f64x8::splat(8.88525527e-09) * t962 * t336 + f64x8::splat(8.88525527e-09) * t384 * t999;
            let t1003 = t284 * t953;
            let t1005 = t286 * t953;
            let t1007 = t281 * t953;
            let t1009 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1003 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1005 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1007;
            let t1017 = t962 * t321;
            let t1036 = -f64x8::splat(6.91592964e-09) * t1009 * t336 - f64x8::splat(6.91592964e-09) * t398 * t999 + f64x8::splat(0.742180708644375) * t954 - f64x8::splat(2.7652468e-07) * t962 * t363 + f64x8::splat(0.00940675747) * t1017 + f64x8::splat(2.36391411e-08) * t1009 * t349 + f64x8::splat(2.36391411e-08) * t398 * t948 + f64x8::splat(6.94482484e-09) * t1009 * t343 + f64x8::splat(6.94482484e-09) * t398 * t985 + f64x8::splat(1.69805915e-07) * t1009 * t363 - f64x8::splat(4.16393106e-08) * t1009 * t354 - f64x8::splat(4.16393106e-08) * t398 * t969 - f64x8::splat(2.65114646e-08) * t1009 * t359 + f64x8::splat(0.48014796319875) * t1003;
            let t1038 = t1009 * t321;
            let t1040 = t398 * t927;
            let t1046 = f64x8::splat(315.0) / f64x8::splat(8.0) * t956 - f64x8::splat(105.0) / f64x8::splat(4.0) * t958 - f64x8::splat(5.0) / f64x8::splat(12.0) * t930 + f64x8::splat(5.0) / f64x8::splat(288.0) * t941;
            let t1065 = t1046 * t321;
            let t1067 = t289 * t927;
            let t1071 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1005 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1007;
            let t1074 = -f64x8::splat(0.00957417512) * t1038 - f64x8::splat(0.00957417512) * t1040 + f64x8::splat(8.50272392e-09) * t1046 * t336 + f64x8::splat(8.50272392e-09) * t289 * t999 - f64x8::splat(2.65114646e-08) * t398 * t976 + f64x8::splat(1.62238741e-07) * t1046 * t354 + f64x8::splat(1.62238741e-07) * t289 * t969 - f64x8::splat(3.76702959e-08) * t1046 * t349 - f64x8::splat(3.76702959e-08) * t289 * t948 - f64x8::splat(1.38472194e-08) * t1046 * t343 - f64x8::splat(1.38472194e-08) * t289 * t985 - f64x8::splat(0.00884148272) * t1065 - f64x8::splat(0.00884148272) * t1067 - f64x8::splat(4.93824365e-09) * t1071 * t336;
            let t1095 = t1071 * t321;
            let t1097 = t326 * t927;
            let t1101 = -f64x8::splat(4.93824365e-09) * t326 * t999 - f64x8::splat(0.00896771404) * t1046 * t359 - f64x8::splat(0.00896771404) * t289 * t976 - f64x8::splat(0.0188495102) * t1046 * t363 + f64x8::splat(9.12223751e-09) * t326 * t985 + f64x8::splat(2.09603871e-08) * t1071 * t349 + f64x8::splat(2.09603871e-08) * t326 * t948 + f64x8::splat(9.12223751e-09) * t1071 * t343 - f64x8::splat(1.975305997940625) * t956 - f64x8::splat(0.18458962865625) * t1007 - f64x8::splat(0.0182911291) * t1071 * t363 + f64x8::splat(0.0162638575) * t1095 + f64x8::splat(0.0162638575) * t1097 - f64x8::splat(7.90811707e-08) * t1071 * t354;
            let t1113 = f64x8::splat(15.0) / f64x8::splat(2.0) * t958 + t930 / f64x8::splat(3.0) - t941 / f64x8::splat(72.0);
            let t1131 = t441 * t927;
            let t1135 = -f64x8::splat(7.90811707e-08) * t326 * t969 + f64x8::splat(0.00631891628) * t1071 * t359 + f64x8::splat(0.00631891628) * t326 * t976 + f64x8::splat(0.00339308972) * t1113 * t354 + f64x8::splat(0.00339308972) * t370 * t969 - f64x8::splat(2.16860568e-08) * t1113 * t343 - f64x8::splat(2.16860568e-08) * t370 * t985 + f64x8::splat(0.000896739466) * t1113 * t349 + f64x8::splat(0.000896739466) * t370 * t948 + f64x8::splat(6.74910119e-09) * t1113 * t336 + f64x8::splat(6.74910119e-09) * t370 * t999 - f64x8::splat(0.5522247359125) * t1005 - f64x8::splat(0.043464346) * t1131 + f64x8::splat(0.00182906057) * t441 * t969;
            let t1143 = t1113 * t321;
            let t1145 = t370 * t927;
            let t1157 = t953 * t343;
            let t1160 = f64x8::splat(6.68980219e-09) * t441 * t985 - f64x8::splat(0.00035104103) * t441 * t948 - f64x8::splat(2.23014657e-09) * t441 * t999 + f64x8::splat(1.493833915228125) * t958 - f64x8::splat(0.0182177954) * t1143 - f64x8::splat(0.0182177954) * t1145 - f64x8::splat(0.00845508103) * t1113 * t359 - f64x8::splat(0.00845508103) * t370 * t976 + f64x8::splat(0.0280678872) * t1113 * t363 - f64x8::splat(0.004729415517815625) * t943 + f64x8::splat(0.013938308465540625) * t945 - f64x8::splat(9.40351563e-06) * t281 * t999 - f64x8::splat(5.14204676e-05) * t1157 - f64x8::splat(0.00048005288013375) * t981;
            let t1166 = t953 * t354;
            let t1170 = t953 * t349;
            let t1174 = t953 * t321;
            let t1176 = t281 * t927;
            let t1178 = t953 * t363;
            let t1180 = t953 * t359;
            let t1186 = t953 * t336;
            let t1188 = f64x8::splat(0.0120828626792125) * t965 + f64x8::splat(0.01406365375513125) * t967 - f64x8::splat(5.14204676e-05) * t281 * t985 + f64x8::splat(0.00119130546) * t1166 + f64x8::splat(0.00119130546) * t281 * t969 + f64x8::splat(0.000822139896) * t1170 + f64x8::splat(0.000822139896) * t281 * t948 + f64x8::splat(0.100339208) * t1174 + f64x8::splat(0.100339208) * t1176 - f64x8::splat(0.00879090772) * t1178 - f64x8::splat(0.00303347141) * t1180 - f64x8::splat(0.00303347141) * t281 * t976 + f64x8::splat(0.00293253041) * t441 * t976 - f64x8::splat(9.40351563e-06) * t1186;
            let t1218 = f64x8::splat(0.000138149743606875) * t994 - f64x8::splat(0.02637272316) * t456 * t927 + f64x8::splat(2.006940657e-08) * t1007 * t343 - f64x8::splat(0.00105312309) * t1007 * t349 + f64x8::splat(0.00548718171) * t1007 * t354 + f64x8::splat(0.00879759123) * t1007 * t359 - f64x8::splat(0.0450310908) * t1007 * t363 - f64x8::splat(0.0450310908) * t454 * t927 - f64x8::splat(0.130393038) * t1007 * t321 + f64x8::splat(0.0842036616) * t438 * t927 - f64x8::splat(6.69043971e-09) * t1007 * t336 - f64x8::splat(0.0548733873) * t366 * t927 - f64x8::splat(0.0565485306) * t322 * t927 + f64x8::splat(5.09417745e-07) * t412 * t927 - f64x8::splat(8.2957404e-07) * t391 * t927;
            let t1221 = t1002 + t1036 + t1074 + t1101 + t1135 + t1160 + t1188 + t1218;
            let t1226 = ((t258).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t885 * t486 - t864 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t267 * t1221));
            let tvrho1 = t257 + t490 + t7 * (t879 + t1226);
            acc_vrho_1 = tvrho1;
            let t1229 = t34 * t39;
            let t1230 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1229;
            let t1231 = ((t62).select(-t1230, f64x8::splat(0.0)));
            let t1234 = t70 * t1231;
            let t1236 = t526 * t1231;
            let t1239 = ((t62).select(f64x8::splat(0.0), -t1230));
            let t1240 = t73 * t1239;
            let t1246 = f64x8::splat(3.0) * t538 * t1239 + f64x8::splat(3.0) * t541 * t1239;
            let t1249 = ((t61).select(f64x8::splat(6.0) * t68 * t1231 + f64x8::splat(3.0) * t1234 - f64x8::splat(12.0) * t1236, f64x8::splat(6.0) * t530 * t1240 + t537 * t1246));
            let t1251 = t557 * v_rho0;
            let t1253 = f64x8::splat(1.0) / t36 / t1251;
            let t1256 = t555 * v_sigma0 * t1253 * t563;
            let t1259 = t34 * t39 * t44;
            let t1263 = t1259 / f64x8::splat(12.0) - t1256 / f64x8::splat(288.0);
            let t1264 = t141 * t1263;
            let t1266 = t143 * t1263;
            let t1268 = t49 * t1263;
            let t1270 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1264 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1266 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1268;
            let t1273 = t97 * t1249;
            let t1275 = t87 * t1249;
            let t1277 = t85 * t1249;
            let t1280 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1273 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1275 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1277 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1249;
            let t1283 = t86 * t1249;
            let t1285 = t84 * t1249;
            let t1287 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1283 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1285;
            let t1292 = t90 * t1249;
            let t1296 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1292 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1283 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1285;
            let t1307 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1277 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1249;
            let t1315 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1275 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1277 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1249;
            let t1318 = -f64x8::splat(0.004373652639371875) * t1249 + f64x8::splat(0.0005425920148328993) * t1256 - f64x8::splat(0.013022208355989584) * t1259 - f64x8::splat(6.91592964e-09) * t1270 * t94 - f64x8::splat(6.91592964e-09) * t210 * t1280 - f64x8::splat(4.16393106e-08) * t210 * t1287 + f64x8::splat(6.94482484e-09) * t1270 * t101 + f64x8::splat(6.94482484e-09) * t210 * t1296 + f64x8::splat(2.36391411e-08) * t1270 * t107 - f64x8::splat(4.16393106e-08) * t1270 * t112 - f64x8::splat(2.65114646e-08) * t1270 * t117 - f64x8::splat(2.65114646e-08) * t210 * t1307 + f64x8::splat(1.69805915e-07) * t1270 * t121 + f64x8::splat(2.36391411e-08) * t210 * t1315;
            let t1319 = t140 * t1263;
            let t1321 = t50 * t1263;
            let t1325 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1319 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1321 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1259 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1256;
            let t1331 = t1270 * t84;
            let t1333 = t210 * t1249;
            let t1353 = -f64x8::splat(1.38472194e-08) * t1325 * t101 - f64x8::splat(1.38472194e-08) * t146 * t1296 + f64x8::splat(0.48014796319875) * t1264 - f64x8::splat(0.00957417512) * t1331 - f64x8::splat(0.00957417512) * t1333 + f64x8::splat(8.50272392e-09) * t1325 * t94 + f64x8::splat(8.50272392e-09) * t146 * t1280 + f64x8::splat(1.62238741e-07) * t146 * t1287 - f64x8::splat(0.00896771404) * t1325 * t117 - f64x8::splat(0.00896771404) * t146 * t1307 - f64x8::splat(3.76702959e-08) * t1325 * t107 - f64x8::splat(3.76702959e-08) * t146 * t1315 + f64x8::splat(1.62238741e-07) * t1325 * t112 + f64x8::splat(9.12223751e-09) * t164 * t1296;
            let t1357 = t1325 * t84;
            let t1359 = t146 * t1249;
            let t1363 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1266 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1268;
            let t1380 = t1363 * t84;
            let t1382 = t164 * t1249;
            let t1387 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1321 - t1259 / f64x8::splat(8.0) + t1256 / f64x8::splat(192.0);
            let t1390 = -f64x8::splat(0.0188495102) * t1325 * t121 - f64x8::splat(0.00884148272) * t1357 - f64x8::splat(0.00884148272) * t1359 - f64x8::splat(4.93824365e-09) * t1363 * t94 - f64x8::splat(4.93824365e-09) * t164 * t1280 - f64x8::splat(7.90811707e-08) * t164 * t1287 + f64x8::splat(0.00631891628) * t1363 * t117 + f64x8::splat(0.00631891628) * t164 * t1307 + f64x8::splat(2.09603871e-08) * t1363 * t107 + f64x8::splat(2.09603871e-08) * t164 * t1315 + f64x8::splat(9.12223751e-09) * t1363 * t101 + f64x8::splat(0.0162638575) * t1380 + f64x8::splat(0.0162638575) * t1382 + f64x8::splat(6.74910119e-09) * t1387 * t94;
            let t1417 = f64x8::splat(6.74910119e-09) * t181 * t1280 - f64x8::splat(7.90811707e-08) * t1363 * t112 - f64x8::splat(0.00845508103) * t181 * t1307 + f64x8::splat(0.000896739466) * t1387 * t107 + f64x8::splat(0.000896739466) * t181 * t1315 - f64x8::splat(2.16860568e-08) * t1387 * t101 - f64x8::splat(2.16860568e-08) * t181 * t1296 - f64x8::splat(0.5522247359125) * t1266 - f64x8::splat(0.18458962865625) * t1268 - f64x8::splat(0.0182911291) * t1363 * t121 + f64x8::splat(8.88525527e-09) * t203 * t1280 + f64x8::splat(0.00339308972) * t1387 * t112 + f64x8::splat(0.00339308972) * t181 * t1287 - f64x8::splat(0.00845508103) * t1387 * t117;
            let t1422 = t206 * t1263;
            let t1428 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1422 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1319 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1321 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1259 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1256;
            let t1449 = t1428 * t84;
            let t1451 = t203 * t1249;
            let t1453 = -f64x8::splat(3.38128188e-08) * t203 * t1315 - f64x8::splat(7.74224962e-09) * t1428 * t101 - f64x8::splat(7.74224962e-09) * t203 * t1296 + f64x8::splat(8.88525527e-09) * t1428 * t94 + f64x8::splat(5.54588743e-08) * t203 * t1287 + f64x8::splat(5.05920757e-08) * t1428 * t117 + f64x8::splat(5.05920757e-08) * t203 * t1307 - f64x8::splat(3.38128188e-08) * t1428 * t107 - f64x8::splat(2.23014657e-09) * t52 * t1280 + f64x8::splat(0.742180708644375) * t1422 - f64x8::splat(1.975305997940625) * t1319 - f64x8::splat(2.7652468e-07) * t1428 * t121 + f64x8::splat(0.00940675747) * t1449 + f64x8::splat(0.00940675747) * t1451;
            let t1460 = t1263 * t101;
            let t1465 = t52 * t1249;
            let t1467 = t1263 * t94;
            let t1478 = f64x8::splat(5.54588743e-08) * t1428 * t112 + f64x8::splat(0.0120828626792125) * t1283 + f64x8::splat(0.01406365375513125) * t1285 + f64x8::splat(0.00119130546) * t49 * t1287 - f64x8::splat(5.14204676e-05) * t1460 - f64x8::splat(0.00048005288013375) * t1292 - f64x8::splat(5.14204676e-05) * t49 * t1296 - f64x8::splat(0.043464346) * t1465 - f64x8::splat(9.40351563e-06) * t1467 + f64x8::splat(0.000138149743606875) * t1273 - f64x8::splat(9.40351563e-06) * t49 * t1280 + f64x8::splat(0.00293253041) * t52 * t1307 - f64x8::splat(0.00035104103) * t52 * t1315 + f64x8::splat(0.00182906057) * t52 * t1287;
            let t1485 = t1387 * t84;
            let t1487 = t181 * t1249;
            let t1489 = t1263 * t84;
            let t1491 = t49 * t1249;
            let t1493 = t1263 * t107;
            let t1498 = t1263 * t112;
            let t1500 = t1263 * t117;
            let t1505 = f64x8::splat(6.68980219e-09) * t52 * t1296 + f64x8::splat(1.493833915228125) * t1321 + f64x8::splat(0.0280678872) * t1387 * t121 - f64x8::splat(0.0182177954) * t1485 - f64x8::splat(0.0182177954) * t1487 + f64x8::splat(0.100339208) * t1489 + f64x8::splat(0.100339208) * t1491 + f64x8::splat(0.000822139896) * t1493 - f64x8::splat(0.004729415517815625) * t1275 + f64x8::splat(0.000822139896) * t49 * t1315 + f64x8::splat(0.00119130546) * t1498 - f64x8::splat(0.00303347141) * t1500 + f64x8::splat(0.013938308465540625) * t1277 - f64x8::splat(0.00303347141) * t49 * t1307;
            let t1506 = t1263 * t121;
            let t1536 = -f64x8::splat(0.00879090772) * t1506 + f64x8::splat(2.006940657e-08) * t1268 * t101 - f64x8::splat(0.00105312309) * t1268 * t107 + f64x8::splat(0.00548718171) * t1268 * t112 + f64x8::splat(0.00879759123) * t1268 * t117 - f64x8::splat(0.0450310908) * t1268 * t121 - f64x8::splat(0.0450310908) * t124 * t1249 - f64x8::splat(0.130393038) * t1268 * t84 + f64x8::splat(0.0842036616) * t196 * t1249 - f64x8::splat(6.69043971e-09) * t1268 * t94 - f64x8::splat(0.0548733873) * t177 * t1249 - f64x8::splat(0.0565485306) * t160 * t1249 + f64x8::splat(5.09417745e-07) * t223 * t1249 - f64x8::splat(8.2957404e-07) * t204 * t1249 - f64x8::splat(0.02637272316) * t138 * t1249;
            let t1539 = t1318 + t1353 + t1390 + t1417 + t1453 + t1478 + t1505 + t1536;
            let t1543 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t1539));
            let tvsigma0 = t7 * t1543;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1544 = t34 * t272;
            let t1545 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1544;
            let t1546 = ((t299).select(-t1545, f64x8::splat(0.0)));
            let t1549 = t307 * t1546;
            let t1551 = t906 * t1546;
            let t1554 = ((t299).select(f64x8::splat(0.0), -t1545));
            let t1555 = t310 * t1554;
            let t1561 = f64x8::splat(3.0) * t918 * t1554 + f64x8::splat(3.0) * t921 * t1554;
            let t1564 = ((t298).select(f64x8::splat(6.0) * t305 * t1546 + f64x8::splat(3.0) * t1549 - f64x8::splat(12.0) * t1551, f64x8::splat(6.0) * t910 * t1555 + t917 * t1561));
            let t1567 = t34 * t272 * t277;
            let t1569 = t933 * v_rho1;
            let t1571 = f64x8::splat(1.0) / t269 / t1569;
            let t1574 = t555 * v_sigma2 * t1571 * t939;
            let t1578 = t1567 / f64x8::splat(12.0) - t1574 / f64x8::splat(288.0);
            let t1579 = t394 * t1578;
            let t1581 = t283 * t1578;
            let t1583 = t282 * t1578;
            let t1587 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1579 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1581 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1583 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1567 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1574;
            let t1590 = t332 * t1564;
            let t1592 = t328 * t1564;
            let t1594 = t321 * t1564;
            let t1596 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1590 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1592 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1594;
            let t1601 = t327 * t1564;
            let t1604 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1601 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1564;
            let t1609 = t329 * t1564;
            let t1613 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1609 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1601 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1564;
            let t1618 = t1587 * t321;
            let t1620 = t384 * t1564;
            let t1626 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1592 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1594;
            let t1629 = -f64x8::splat(0.004373652639371875) * t1564 - f64x8::splat(0.013022208355989584) * t1567 + f64x8::splat(0.0005425920148328993) * t1574 - f64x8::splat(7.74224962e-09) * t1587 * t343 - f64x8::splat(7.74224962e-09) * t384 * t1596 + f64x8::splat(5.05920757e-08) * t1587 * t359 + f64x8::splat(5.05920757e-08) * t384 * t1604 - f64x8::splat(3.38128188e-08) * t1587 * t349 - f64x8::splat(3.38128188e-08) * t384 * t1613 - f64x8::splat(2.7652468e-07) * t1587 * t363 + f64x8::splat(0.00940675747) * t1618 + f64x8::splat(0.00940675747) * t1620 + f64x8::splat(5.54588743e-08) * t1587 * t354 + f64x8::splat(5.54588743e-08) * t384 * t1626;
            let t1630 = t284 * t1578;
            let t1632 = t286 * t1578;
            let t1634 = t281 * t1578;
            let t1636 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1630 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1632 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1634;
            let t1645 = t339 * t1564;
            let t1650 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1645 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1609 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1601 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1564;
            let t1661 = t398 * t1564;
            let t1667 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1581 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1583 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1567 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1574;
            let t1676 = f64x8::splat(6.94482484e-09) * t1636 * t343 + f64x8::splat(6.94482484e-09) * t398 * t1596 + f64x8::splat(2.36391411e-08) * t1636 * t349 - f64x8::splat(6.91592964e-09) * t1636 * t336 - f64x8::splat(6.91592964e-09) * t398 * t1650 + f64x8::splat(1.69805915e-07) * t1636 * t363 + f64x8::splat(2.36391411e-08) * t398 * t1613 - f64x8::splat(4.16393106e-08) * t1636 * t354 - f64x8::splat(4.16393106e-08) * t398 * t1626 - f64x8::splat(0.00957417512) * t1661 + f64x8::splat(8.50272392e-09) * t1667 * t336 + f64x8::splat(8.50272392e-09) * t289 * t1650 - f64x8::splat(2.65114646e-08) * t1636 * t359 - f64x8::splat(2.65114646e-08) * t398 * t1604;
            let t1678 = t281 * t1564;
            let t1680 = t1578 * t363;
            let t1682 = t1578 * t359;
            let t1690 = t1636 * t321;
            let t1702 = t1578 * t321;
            let t1706 = f64x8::splat(0.100339208) * t1678 - f64x8::splat(0.00879090772) * t1680 - f64x8::splat(0.00303347141) * t1682 - f64x8::splat(0.00303347141) * t281 * t1604 - f64x8::splat(1.38472194e-08) * t1667 * t343 - f64x8::splat(1.38472194e-08) * t289 * t1596 - f64x8::splat(0.00957417512) * t1690 + f64x8::splat(1.62238741e-07) * t289 * t1626 - f64x8::splat(0.00896771404) * t1667 * t359 - f64x8::splat(0.00896771404) * t289 * t1604 - f64x8::splat(3.76702959e-08) * t1667 * t349 - f64x8::splat(3.76702959e-08) * t289 * t1613 + f64x8::splat(0.100339208) * t1702 - f64x8::splat(0.0188495102) * t1667 * t363;
            let t1707 = t1667 * t321;
            let t1709 = t289 * t1564;
            let t1713 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1632 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1634;
            let t1736 = t1578 * t354;
            let t1738 = -f64x8::splat(0.00884148272) * t1707 - f64x8::splat(0.00884148272) * t1709 - f64x8::splat(4.93824365e-09) * t1713 * t336 - f64x8::splat(4.93824365e-09) * t326 * t1650 + f64x8::splat(1.62238741e-07) * t1667 * t354 + f64x8::splat(0.00631891628) * t1713 * t359 + f64x8::splat(0.00631891628) * t326 * t1604 + f64x8::splat(2.09603871e-08) * t1713 * t349 + f64x8::splat(2.09603871e-08) * t326 * t1613 + f64x8::splat(9.12223751e-09) * t1713 * t343 + f64x8::splat(9.12223751e-09) * t326 * t1596 + f64x8::splat(8.88525527e-09) * t1587 * t336 + f64x8::splat(8.88525527e-09) * t384 * t1650 + f64x8::splat(0.00119130546) * t1736;
            let t1743 = t1578 * t349;
            let t1754 = t1713 * t321;
            let t1756 = t326 * t1564;
            let t1761 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1583 - t1567 / f64x8::splat(8.0) + t1574 / f64x8::splat(192.0);
            let t1769 = f64x8::splat(0.00119130546) * t281 * t1626 + f64x8::splat(0.000822139896) * t1743 + f64x8::splat(0.000822139896) * t281 * t1613 - f64x8::splat(7.90811707e-08) * t1713 * t354 - f64x8::splat(7.90811707e-08) * t326 * t1626 - f64x8::splat(0.18458962865625) * t1634 - f64x8::splat(0.0182911291) * t1713 * t363 + f64x8::splat(0.0162638575) * t1754 + f64x8::splat(0.0162638575) * t1756 + f64x8::splat(6.74910119e-09) * t1761 * t336 + f64x8::splat(0.742180708644375) * t1579 + f64x8::splat(0.48014796319875) * t1630 - f64x8::splat(1.975305997940625) * t1581 - f64x8::splat(0.00845508103) * t370 * t1604;
            let t1774 = t1578 * t343;
            let t1778 = t1578 * t336;
            let t1791 = t1761 * t321;
            let t1793 = t370 * t1564;
            let t1797 = f64x8::splat(0.000896739466) * t1761 * t349 + f64x8::splat(0.000896739466) * t370 * t1613 - f64x8::splat(5.14204676e-05) * t1774 - f64x8::splat(5.14204676e-05) * t281 * t1596 - f64x8::splat(9.40351563e-06) * t1778 - f64x8::splat(9.40351563e-06) * t281 * t1650 + f64x8::splat(6.74910119e-09) * t370 * t1650 - f64x8::splat(2.16860568e-08) * t1761 * t343 - f64x8::splat(2.16860568e-08) * t370 * t1596 - f64x8::splat(0.5522247359125) * t1632 + f64x8::splat(0.0280678872) * t1761 * t363 - f64x8::splat(0.0182177954) * t1791 - f64x8::splat(0.0182177954) * t1793 + f64x8::splat(0.00339308972) * t1761 * t354;
            let t1803 = t441 * t1564;
            let t1821 = f64x8::splat(0.00339308972) * t370 * t1626 - f64x8::splat(0.00845508103) * t1761 * t359 - f64x8::splat(0.043464346) * t1803 + f64x8::splat(0.013938308465540625) * t1601 + f64x8::splat(0.00293253041) * t441 * t1604 - f64x8::splat(0.004729415517815625) * t1609 - f64x8::splat(0.00035104103) * t441 * t1613 + f64x8::splat(0.0120828626792125) * t1592 + f64x8::splat(0.01406365375513125) * t1594 + f64x8::splat(0.00182906057) * t441 * t1626 - f64x8::splat(0.00048005288013375) * t1590 + f64x8::splat(6.68980219e-09) * t441 * t1596 + f64x8::splat(0.000138149743606875) * t1645 - f64x8::splat(2.23014657e-09) * t441 * t1650;
            let t1851 = f64x8::splat(1.493833915228125) * t1583 - f64x8::splat(0.0450310908) * t1634 * t363 - f64x8::splat(0.0450310908) * t454 * t1564 - f64x8::splat(0.130393038) * t1634 * t321 - f64x8::splat(0.02637272316) * t456 * t1564 - f64x8::splat(6.69043971e-09) * t1634 * t336 + f64x8::splat(2.006940657e-08) * t1634 * t343 - f64x8::splat(0.00105312309) * t1634 * t349 + f64x8::splat(0.00548718171) * t1634 * t354 + f64x8::splat(0.00879759123) * t1634 * t359 + f64x8::splat(0.0842036616) * t438 * t1564 - f64x8::splat(0.0548733873) * t366 * t1564 - f64x8::splat(0.0565485306) * t322 * t1564 + f64x8::splat(5.09417745e-07) * t412 * t1564 - f64x8::splat(8.2957404e-07) * t391 * t1564;
            let t1854 = t1629 + t1676 + t1706 + t1738 + t1769 + t1797 + t1821 + t1851;
            let t1858 = ((t258).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t267 * t1854));
            let tvsigma2 = t7 * t1858;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t1861 = f64x8::splat(5.0) / f64x8::splat(9.0) * t54 * t29 * t33;
            let t1862 = ((t62).select(t1861, f64x8::splat(0.0)));
            let t1865 = t70 * t1862;
            let t1867 = t526 * t1862;
            let t1870 = ((t62).select(f64x8::splat(0.0), t1861));
            let t1871 = t73 * t1870;
            let t1877 = f64x8::splat(3.0) * t538 * t1870 + f64x8::splat(3.0) * t541 * t1870;
            let t1880 = ((t61).select(f64x8::splat(6.0) * t68 * t1862 + f64x8::splat(3.0) * t1865 - f64x8::splat(12.0) * t1867, f64x8::splat(6.0) * t530 * t1871 + t537 * t1877));
            let t1882 = t90 * t1880;
            let t1884 = t86 * t1880;
            let t1886 = t84 * t1880;
            let t1888 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1882 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1884 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1886;
            let t1891 = t87 * t1880;
            let t1893 = t85 * t1880;
            let t1896 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1891 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1893 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1880;
            let t1899 = t97 * t1880;
            let t1904 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1899 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1891 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1893 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1880;
            let t1915 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1893 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1880;
            let t1918 = t203 * t1880;
            let t1922 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1884 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1886;
            let t1929 = t210 * t1880;
            let t1933 = -f64x8::splat(0.004373652639371875) * t1880 - f64x8::splat(7.74224962e-09) * t203 * t1888 - f64x8::splat(3.38128188e-08) * t203 * t1896 + f64x8::splat(8.88525527e-09) * t203 * t1904 + f64x8::splat(6.94482484e-09) * t210 * t1888 + f64x8::splat(2.36391411e-08) * t210 * t1896 - f64x8::splat(6.91592964e-09) * t210 * t1904 + f64x8::splat(5.05920757e-08) * t203 * t1915 + f64x8::splat(0.00940675747) * t1918 + f64x8::splat(5.54588743e-08) * t203 * t1922 + f64x8::splat(8.50272392e-09) * t146 * t1904 - f64x8::splat(2.65114646e-08) * t210 * t1915 - f64x8::splat(0.00957417512) * t1929 - f64x8::splat(4.16393106e-08) * t210 * t1922;
            let t1936 = t146 * t1880;
            let t1952 = t181 * t1880;
            let t1962 = f64x8::splat(9.12223751e-09) * t164 * t1888 - f64x8::splat(0.00884148272) * t1936 - f64x8::splat(4.93824365e-09) * t164 * t1904 + f64x8::splat(1.62238741e-07) * t146 * t1922 - f64x8::splat(0.00896771404) * t146 * t1915 - f64x8::splat(3.76702959e-08) * t146 * t1896 - f64x8::splat(1.38472194e-08) * t146 * t1888 - f64x8::splat(2.23014657e-09) * t52 * t1904 + f64x8::splat(6.68980219e-09) * t52 * t1888 - f64x8::splat(0.0182177954) * t1952 + f64x8::splat(0.00339308972) * t181 * t1922 - f64x8::splat(0.00845508103) * t181 * t1915 + f64x8::splat(0.000896739466) * t181 * t1896 - f64x8::splat(2.16860568e-08) * t181 * t1888;
            let t1964 = t164 * t1880;
            let t1974 = t49 * t1880;
            let t1987 = f64x8::splat(0.0162638575) * t1964 + f64x8::splat(6.74910119e-09) * t181 * t1904 - f64x8::splat(7.90811707e-08) * t164 * t1922 + f64x8::splat(0.00631891628) * t164 * t1915 + f64x8::splat(2.09603871e-08) * t164 * t1896 + f64x8::splat(0.100339208) * t1974 + f64x8::splat(0.0120828626792125) * t1884 + f64x8::splat(0.01406365375513125) * t1886 + f64x8::splat(0.00119130546) * t49 * t1922 + f64x8::splat(0.013938308465540625) * t1893 - f64x8::splat(0.00303347141) * t49 * t1915 - f64x8::splat(0.004729415517815625) * t1891 + f64x8::splat(0.000822139896) * t49 * t1896 - f64x8::splat(0.00048005288013375) * t1882;
            let t1990 = t52 * t1880;
            let t2015 = -f64x8::splat(5.14204676e-05) * t49 * t1888 - f64x8::splat(0.043464346) * t1990 + f64x8::splat(0.000138149743606875) * t1899 - f64x8::splat(9.40351563e-06) * t49 * t1904 + f64x8::splat(0.00182906057) * t52 * t1922 + f64x8::splat(0.00293253041) * t52 * t1915 - f64x8::splat(0.00035104103) * t52 * t1896 - f64x8::splat(0.02637272316) * t138 * t1880 - f64x8::splat(0.0450310908) * t124 * t1880 - f64x8::splat(0.0548733873) * t177 * t1880 + f64x8::splat(0.0842036616) * t196 * t1880 - f64x8::splat(0.0565485306) * t160 * t1880 + f64x8::splat(5.09417745e-07) * t223 * t1880 - f64x8::splat(8.2957404e-07) * t204 * t1880;
            let t2017 = t1933 + t1962 + t1987 + t2015;
            let t2021 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t2017));
            let tvtau0 = t7 * t2021;
            acc_vtau_0 = tvtau0;
            let t2024 = f64x8::splat(5.0) / f64x8::splat(9.0) * t291 * t29 * t33;
            let t2025 = ((t299).select(t2024, f64x8::splat(0.0)));
            let t2028 = t307 * t2025;
            let t2030 = t906 * t2025;
            let t2033 = ((t299).select(f64x8::splat(0.0), t2024));
            let t2034 = t310 * t2033;
            let t2040 = f64x8::splat(3.0) * t918 * t2033 + f64x8::splat(3.0) * t921 * t2033;
            let t2043 = ((t298).select(f64x8::splat(6.0) * t305 * t2025 + f64x8::splat(3.0) * t2028 - f64x8::splat(12.0) * t2030, f64x8::splat(6.0) * t910 * t2034 + t917 * t2040));
            let t2045 = t339 * t2043;
            let t2047 = t329 * t2043;
            let t2049 = t327 * t2043;
            let t2052 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t2045 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t2047 + f64x8::splat(945.0) / f64x8::splat(16.0) * t2049 - f64x8::splat(35.0) / f64x8::splat(16.0) * t2043;
            let t2055 = t332 * t2043;
            let t2057 = t328 * t2043;
            let t2059 = t321 * t2043;
            let t2061 = f64x8::splat(693.0) / f64x8::splat(8.0) * t2055 - f64x8::splat(315.0) / f64x8::splat(4.0) * t2057 + f64x8::splat(105.0) / f64x8::splat(8.0) * t2059;
            let t2068 = f64x8::splat(35.0) / f64x8::splat(2.0) * t2057 - f64x8::splat(15.0) / f64x8::splat(2.0) * t2059;
            let t2073 = f64x8::splat(15.0) / f64x8::splat(2.0) * t2049 - f64x8::splat(3.0) / f64x8::splat(2.0) * t2043;
            let t2079 = f64x8::splat(315.0) / f64x8::splat(8.0) * t2047 - f64x8::splat(105.0) / f64x8::splat(4.0) * t2049 + f64x8::splat(15.0) / f64x8::splat(8.0) * t2043;
            let t2088 = t384 * t2043;
            let t2094 = t398 * t2043;
            let t2096 = -f64x8::splat(0.004373652639371875) * t2043 + f64x8::splat(8.88525527e-09) * t384 * t2052 - f64x8::splat(7.74224962e-09) * t384 * t2061 - f64x8::splat(6.91592964e-09) * t398 * t2052 + f64x8::splat(5.54588743e-08) * t384 * t2068 + f64x8::splat(5.05920757e-08) * t384 * t2073 - f64x8::splat(3.38128188e-08) * t384 * t2079 + f64x8::splat(2.36391411e-08) * t398 * t2079 - f64x8::splat(4.16393106e-08) * t398 * t2068 + f64x8::splat(6.94482484e-09) * t398 * t2061 + f64x8::splat(0.00940675747) * t2088 - f64x8::splat(1.38472194e-08) * t289 * t2061 - f64x8::splat(2.65114646e-08) * t398 * t2073 - f64x8::splat(0.00957417512) * t2094;
            let t2101 = t289 * t2043;
            let t2115 = t370 * t2043;
            let t2125 = -f64x8::splat(7.90811707e-08) * t326 * t2068 + f64x8::splat(9.12223751e-09) * t326 * t2061 - f64x8::splat(0.00884148272) * t2101 - f64x8::splat(4.93824365e-09) * t326 * t2052 + f64x8::splat(1.62238741e-07) * t289 * t2068 - f64x8::splat(0.00896771404) * t289 * t2073 - f64x8::splat(3.76702959e-08) * t289 * t2079 + f64x8::splat(8.50272392e-09) * t289 * t2052 + f64x8::splat(6.68980219e-09) * t441 * t2061 - f64x8::splat(0.0182177954) * t2115 + f64x8::splat(0.00339308972) * t370 * t2068 - f64x8::splat(0.00845508103) * t370 * t2073 + f64x8::splat(0.000896739466) * t370 * t2079 + f64x8::splat(6.74910119e-09) * t370 * t2052;
            let t2131 = t326 * t2043;
            let t2149 = -f64x8::splat(2.16860568e-08) * t370 * t2061 + f64x8::splat(0.00631891628) * t326 * t2073 + f64x8::splat(0.0162638575) * t2131 + f64x8::splat(2.09603871e-08) * t326 * t2079 + f64x8::splat(0.000138149743606875) * t2045 - f64x8::splat(0.004729415517815625) * t2047 + f64x8::splat(0.013938308465540625) * t2049 - f64x8::splat(9.40351563e-06) * t281 * t2052 - f64x8::splat(0.00048005288013375) * t2055 + f64x8::splat(0.0120828626792125) * t2057 + f64x8::splat(0.01406365375513125) * t2059 - f64x8::splat(5.14204676e-05) * t281 * t2061 + f64x8::splat(0.00119130546) * t281 * t2068 + f64x8::splat(0.000822139896) * t281 * t2079;
            let t2150 = t281 * t2043;
            let t2154 = t441 * t2043;
            let t2178 = f64x8::splat(0.100339208) * t2150 - f64x8::splat(0.00303347141) * t281 * t2073 - f64x8::splat(0.043464346) * t2154 + f64x8::splat(0.00182906057) * t441 * t2068 + f64x8::splat(0.00293253041) * t441 * t2073 - f64x8::splat(0.00035104103) * t441 * t2079 - f64x8::splat(2.23014657e-09) * t441 * t2052 - f64x8::splat(0.0450310908) * t454 * t2043 - f64x8::splat(0.02637272316) * t456 * t2043 + f64x8::splat(0.0842036616) * t438 * t2043 - f64x8::splat(0.0548733873) * t366 * t2043 - f64x8::splat(0.0565485306) * t322 * t2043 + f64x8::splat(5.09417745e-07) * t412 * t2043 - f64x8::splat(8.2957404e-07) * t391 * t2043;
            let t2180 = t2096 + t2125 + t2149 + t2178;
            let t2184 = ((t258).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t267 * t2180));
            let tvtau1 = t7 * t2184;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
