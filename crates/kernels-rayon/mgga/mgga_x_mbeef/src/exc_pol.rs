//! MGGA_X_MBEEF exc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_mbeef_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
