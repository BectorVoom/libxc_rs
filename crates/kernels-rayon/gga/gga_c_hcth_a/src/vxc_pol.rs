//! GGA_C_HCTH_A vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_hcth_a.c`
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
pub fn gga_c_hcth_a_vxc_pol(
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
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = f64x8::splat(1.0) + t5;
            let t7 = (t6).simd_le(zeta_threshold);
            let t8 = ((v_rho0).simd_le(dens_threshold)) | (t7);
            let t9 = ((t7).select(zeta_threshold, t6));
            let t10 = f64x8::splat(M_CBRT3);
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = t10 * t12;
            let t14 = f64x8::splat(M_CBRT4);
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = (simd::cbrt(t3));
            let t18 = f64x8::splat(1.0) / t17;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = (simd::cbrt(t6));
            let t25 = ((t7).select(t22, f64x8::splat(1.0) / t23));
            let t27 = t16 * t20 * t25;
            let t28 = t27 / f64x8::splat(4.0);
            let t29 = ((t27).sqrt());
            let t31 = t28 + f64x8::splat(1.86372) * t29 + f64x8::splat(12.9352);
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t25 * t32;
            let t37 = (simd::ln(t16 * t20 * t33 / f64x8::splat(4.0)));
            let t38 = f64x8::splat(0.0310907) * t37;
            let t39 = t29 + f64x8::splat(3.72744);
            let t42 = (simd::atan(f64x8::splat(6.15199081975908) / t39));
            let t43 = f64x8::splat(0.038783294878113016) * t42;
            let t44 = t29 / f64x8::splat(2.0);
            let t45 = t44 + f64x8::splat(0.10498);
            let t46 = t45 * t45;
            let t48 = (simd::ln(t46 * t32));
            let t49 = f64x8::splat(0.0009690227711544374) * t48;
            let t51 = t28 + f64x8::splat(3.53021) * t29 + f64x8::splat(18.0578);
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t25 * t52;
            let t57 = (simd::ln(t16 * t20 * t53 / f64x8::splat(4.0)));
            let t59 = t29 + f64x8::splat(7.06042);
            let t62 = (simd::atan(f64x8::splat(4.730926909560113) / t59));
            let t64 = t44 + f64x8::splat(0.325);
            let t65 = t64 * t64;
            let t67 = (simd::ln(t65 * t52));
            let t71 = t21 * zeta_threshold;
            let t73 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t71, f64x8::splat(2.0) * t19));
            let t75 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t71, f64x8::splat(0.0)));
            let t76 = t73 + t75 - f64x8::splat(2.0);
            let t78 = t19 - f64x8::splat(1.0);
            let t80 = f64x8::splat(1.0) / t78 / f64x8::splat(2.0);
            let t82 = t38 + t43 + t49 + (f64x8::splat(0.01554535) * t57 + f64x8::splat(0.05249139316978094) * t62 + f64x8::splat(0.0022478670955426118) * t67 - t38 - t43 - t49) * t76 * t80;
            let t85 = ((t8).select(f64x8::splat(0.0), t9 * t82 / f64x8::splat(2.0)));
            let t86 = v_rho0 * v_rho0;
            let t87 = (simd::cbrt(v_rho0));
            let t88 = t87 * t87;
            let t90 = f64x8::splat(1.0) / t88 / t86;
            let t91 = v_sigma0 * t90;
            let t93 = f64x8::splat(1.0) + f64x8::splat(0.2) * t91;
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = v_sigma0 * v_sigma0;
            let t98 = t86 * t86;
            let t99 = t98 * v_rho0;
            let t101 = f64x8::splat(1.0) / t87 / t99;
            let t103 = t93 * t93;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t97 * v_sigma0;
            let t108 = t98 * t98;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t103 * t93;
            let t112 = f64x8::splat(1.0) / t111;
            let t115 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t91 * t94 - f64x8::splat(0.02203076) * t97 * t101 * t104 + f64x8::splat(0.00831576) * t107 * t109 * t112;
            let t116 = t85 * t115;
            let t118 = f64x8::splat(1.0) - t5;
            let t119 = (t118).simd_le(zeta_threshold);
            let t120 = ((v_rho1).simd_le(dens_threshold)) | (t119);
            let t121 = ((t119).select(zeta_threshold, t118));
            let t122 = (simd::cbrt(t118));
            let t124 = ((t119).select(t22, f64x8::splat(1.0) / t122));
            let t126 = t16 * t20 * t124;
            let t127 = t126 / f64x8::splat(4.0);
            let t128 = ((t126).sqrt());
            let t130 = t127 + f64x8::splat(1.86372) * t128 + f64x8::splat(12.9352);
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t124 * t131;
            let t136 = (simd::ln(t16 * t20 * t132 / f64x8::splat(4.0)));
            let t137 = f64x8::splat(0.0310907) * t136;
            let t138 = t128 + f64x8::splat(3.72744);
            let t141 = (simd::atan(f64x8::splat(6.15199081975908) / t138));
            let t142 = f64x8::splat(0.038783294878113016) * t141;
            let t143 = t128 / f64x8::splat(2.0);
            let t144 = t143 + f64x8::splat(0.10498);
            let t145 = t144 * t144;
            let t147 = (simd::ln(t145 * t131));
            let t148 = f64x8::splat(0.0009690227711544374) * t147;
            let t150 = t127 + f64x8::splat(3.53021) * t128 + f64x8::splat(18.0578);
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t124 * t151;
            let t156 = (simd::ln(t16 * t20 * t152 / f64x8::splat(4.0)));
            let t158 = t128 + f64x8::splat(7.06042);
            let t161 = (simd::atan(f64x8::splat(4.730926909560113) / t158));
            let t163 = t143 + f64x8::splat(0.325);
            let t164 = t163 * t163;
            let t166 = (simd::ln(t164 * t151));
            let t171 = t137 + t142 + t148 + (f64x8::splat(0.01554535) * t156 + f64x8::splat(0.05249139316978094) * t161 + f64x8::splat(0.0022478670955426118) * t166 - t137 - t142 - t148) * t76 * t80;
            let t174 = ((t120).select(f64x8::splat(0.0), t121 * t171 / f64x8::splat(2.0)));
            let t175 = v_rho1 * v_rho1;
            let t176 = (simd::cbrt(v_rho1));
            let t177 = t176 * t176;
            let t179 = f64x8::splat(1.0) / t177 / t175;
            let t180 = v_sigma2 * t179;
            let t182 = f64x8::splat(1.0) + f64x8::splat(0.2) * t180;
            let t183 = f64x8::splat(1.0) / t182;
            let t186 = v_sigma2 * v_sigma2;
            let t187 = t175 * t175;
            let t188 = t187 * v_rho1;
            let t190 = f64x8::splat(1.0) / t176 / t188;
            let t192 = t182 * t182;
            let t193 = f64x8::splat(1.0) / t192;
            let t196 = t186 * v_sigma2;
            let t197 = t187 * t187;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = t192 * t182;
            let t201 = f64x8::splat(1.0) / t200;
            let t204 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t180 * t183 - f64x8::splat(0.02203076) * t186 * t190 * t193 + f64x8::splat(0.00831576) * t196 * t198 * t201;
            let t205 = t174 * t204;
            let t206 = t15 * t18;
            let t207 = t13 * t206;
            let t208 = t207 / f64x8::splat(4.0);
            let t209 = ((t207).sqrt());
            let t211 = t208 + f64x8::splat(1.86372) * t209 + f64x8::splat(12.9352);
            let t212 = f64x8::splat(1.0) / t211;
            let t216 = (simd::ln(t13 * t206 * t212 / f64x8::splat(4.0)));
            let t217 = f64x8::splat(0.0310907) * t216;
            let t218 = t209 + f64x8::splat(3.72744);
            let t221 = (simd::atan(f64x8::splat(6.15199081975908) / t218));
            let t222 = f64x8::splat(0.038783294878113016) * t221;
            let t223 = t209 / f64x8::splat(2.0);
            let t224 = t223 + f64x8::splat(0.10498);
            let t225 = t224 * t224;
            let t227 = (simd::ln(t225 * t212));
            let t228 = f64x8::splat(0.0009690227711544374) * t227;
            let t229 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t230 = f64x8::splat(1.0) / t229;
            let t232 = t208 + f64x8::splat(0.565535) * t209 + f64x8::splat(13.0045);
            let t233 = f64x8::splat(1.0) / t232;
            let t237 = (simd::ln(t13 * t206 * t233 / f64x8::splat(4.0)));
            let t238 = t209 + f64x8::splat(1.13107);
            let t241 = (simd::atan(f64x8::splat(7.123108917818118) / t238));
            let t243 = t223 + f64x8::splat(0.0047584);
            let t244 = t243 * t243;
            let t246 = (simd::ln(t244 * t233));
            let t249 = t230 * (t237 + f64x8::splat(0.31770800474394145) * t241 + f64x8::splat(0.00041403379428206277) * t246);
            let t250 = t23 * t6;
            let t251 = ((t7).select(t71, t250));
            let t252 = t122 * t118;
            let t253 = ((t119).select(t71, t252));
            let t254 = t251 + t253 - f64x8::splat(2.0);
            let t255 = t249 * t254;
            let t256 = t2 * t2;
            let t257 = t256 * t256;
            let t258 = t3 * t3;
            let t259 = t258 * t258;
            let t260 = f64x8::splat(1.0) / t259;
            let t264 = f64x8::splat(9.0) * t78;
            let t265 = t80 * (-t257 * t260 + f64x8::splat(1.0)) * t264;
            let t269 = t208 + f64x8::splat(3.53021) * t209 + f64x8::splat(18.0578);
            let t270 = f64x8::splat(1.0) / t269;
            let t274 = (simd::ln(t13 * t206 * t270 / f64x8::splat(4.0)));
            let t276 = t209 + f64x8::splat(7.06042);
            let t279 = (simd::atan(f64x8::splat(4.730926909560113) / t276));
            let t281 = t223 + f64x8::splat(0.325);
            let t282 = t281 * t281;
            let t284 = (simd::ln(t282 * t270));
            let t286 = f64x8::splat(0.01554535) * t274 + f64x8::splat(0.05249139316978094) * t279 + f64x8::splat(0.0022478670955426118) * t284 - t217 - t222 - t228;
            let t287 = t286 * t254;
            let t288 = t80 * t257;
            let t289 = t288 * t260;
            let t291 = t217 + t222 + t228 - t255 * t265 / f64x8::splat(24.0) + t287 * t289 - t85 - t174;
            let t292 = t91 + t180;
            let t295 = f64x8::splat(1.0) + f64x8::splat(0.003) * t91 + f64x8::splat(0.003) * t180;
            let t296 = f64x8::splat(1.0) / t295;
            let t299 = t292 * t292;
            let t300 = t295 * t295;
            let t301 = f64x8::splat(1.0) / t300;
            let t304 = t299 * t292;
            let t305 = t300 * t295;
            let t306 = f64x8::splat(1.0) / t305;
            let t309 = f64x8::splat(0.836897) + f64x8::splat(0.00516153) * t292 * t296 - f64x8::splat(2.506482e-05) * t299 * t301 - f64x8::splat(1.2352608e-07) * t304 * t306;
            let t310 = t291 * t309;
            let tzk0 = t116 + t205 + t310;
            acc_zk = tzk0;
            let t311 = f64x8::splat(1.0) / t258;
            let t312 = t2 * t311;
            let t313 = t4 - t312;
            let t314 = ((t7).select(f64x8::splat(0.0), t313));
            let t317 = f64x8::splat(1.0) / t17 / t3;
            let t318 = t317 * t19;
            let t321 = t16 * t318 * t33 / f64x8::splat(12.0);
            let t322 = f64x8::splat(1.0) / t250;
            let t325 = ((t7).select(f64x8::splat(0.0), -t322 * t313 / f64x8::splat(3.0)));
            let t326 = t325 * t32;
            let t330 = t19 * t25;
            let t331 = t31 * t31;
            let t332 = f64x8::splat(1.0) / t331;
            let t334 = t16 * t318 * t25;
            let t335 = t334 / f64x8::splat(12.0);
            let t337 = t16 * t20 * t325;
            let t338 = t337 / f64x8::splat(4.0);
            let t339 = f64x8::splat(1.0) / t29;
            let t340 = t334 / f64x8::splat(3.0);
            let t341 = -t340 + t337;
            let t342 = t339 * t341;
            let t344 = -t335 + t338 + f64x8::splat(0.93186) * t342;
            let t345 = t332 * t344;
            let t346 = t330 * t345;
            let t350 = t10 * t10;
            let t351 = (-t321 + t16 * t20 * t326 / f64x8::splat(4.0) - t207 * t346 / f64x8::splat(4.0)) * t350;
            let t352 = f64x8::splat(1.0) / t12;
            let t353 = t352 * t14;
            let t354 = t351 * t353;
            let t355 = t19 * t19;
            let t356 = t17 * t355;
            let t357 = f64x8::splat(1.0) / t25;
            let t358 = t357 * t31;
            let t359 = t356 * t358;
            let t361 = f64x8::splat(0.005181783333333334) * t354 * t359;
            let t362 = t39 * t39;
            let t363 = f64x8::splat(1.0) / t362;
            let t364 = t363 * t339;
            let t366 = f64x8::splat(37.8469910464) * t363 + f64x8::splat(1.0);
            let t367 = f64x8::splat(1.0) / t366;
            let t368 = t341 * t367;
            let t370 = f64x8::splat(0.11929723702508031) * t364 * t368;
            let t371 = t45 * t32;
            let t374 = t46 * t332;
            let t376 = t371 * t342 / f64x8::splat(2.0) - t374 * t344;
            let t377 = f64x8::splat(1.0) / t46;
            let t378 = t376 * t377;
            let t380 = f64x8::splat(0.0009690227711544374) * t378 * t31;
            let t383 = t16 * t318 * t53 / f64x8::splat(12.0);
            let t384 = t325 * t52;
            let t388 = t51 * t51;
            let t389 = f64x8::splat(1.0) / t388;
            let t391 = -t335 + t338 + f64x8::splat(1.765105) * t342;
            let t392 = t389 * t391;
            let t393 = t330 * t392;
            let t397 = (-t383 + t16 * t20 * t384 / f64x8::splat(4.0) - t207 * t393 / f64x8::splat(4.0)) * t350;
            let t398 = t397 * t353;
            let t399 = t357 * t51;
            let t400 = t356 * t399;
            let t403 = t59 * t59;
            let t404 = f64x8::splat(1.0) / t403;
            let t405 = t404 * t339;
            let t407 = f64x8::splat(22.3816694236) * t404 + f64x8::splat(1.0);
            let t408 = f64x8::splat(1.0) / t407;
            let t409 = t341 * t408;
            let t412 = t64 * t52;
            let t415 = t65 * t389;
            let t417 = t412 * t342 / f64x8::splat(2.0) - t415 * t391;
            let t418 = f64x8::splat(1.0) / t65;
            let t419 = t417 * t418;
            let t425 = t361 - t370 + t380 + (f64x8::splat(0.002590891666666667) * t398 * t400 - f64x8::splat(0.12416647223360827) * t405 * t409 + f64x8::splat(0.0022478670955426118) * t419 * t51 - t361 + t370 - t380) * t76 * t80;
            let t429 = ((t8).select(f64x8::splat(0.0), t314 * t82 / f64x8::splat(2.0) + t9 * t425 / f64x8::splat(2.0)));
            let t430 = t429 * t115;
            let t431 = t86 * v_rho0;
            let t433 = f64x8::splat(1.0) / t88 / t431;
            let t434 = v_sigma0 * t433;
            let t437 = t98 * t86;
            let t439 = f64x8::splat(1.0) / t87 / t437;
            let t443 = t108 * v_rho0;
            let t444 = f64x8::splat(1.0) / t443;
            let t448 = t97 * t97;
            let t449 = t108 * t431;
            let t451 = f64x8::splat(1.0) / t88 / t449;
            let t453 = t103 * t103;
            let t454 = f64x8::splat(1.0) / t453;
            let t457 = -f64x8::splat(0.143424) * t434 * t94 + f64x8::splat(0.14618218666666666) * t97 * t439 * t104 - f64x8::splat(0.09002555733333334) * t107 * t444 * t112 + f64x8::splat(0.013305216) * t448 * t451 * t454;
            let t458 = t85 * t457;
            let t459 = -t313;
            let t460 = ((t119).select(f64x8::splat(0.0), t459));
            let t464 = t16 * t318 * t132 / f64x8::splat(12.0);
            let t465 = f64x8::splat(1.0) / t252;
            let t468 = ((t119).select(f64x8::splat(0.0), -t465 * t459 / f64x8::splat(3.0)));
            let t469 = t468 * t131;
            let t473 = t19 * t124;
            let t474 = t130 * t130;
            let t475 = f64x8::splat(1.0) / t474;
            let t477 = t16 * t318 * t124;
            let t478 = t477 / f64x8::splat(12.0);
            let t480 = t16 * t20 * t468;
            let t481 = t480 / f64x8::splat(4.0);
            let t482 = f64x8::splat(1.0) / t128;
            let t483 = t477 / f64x8::splat(3.0);
            let t484 = -t483 + t480;
            let t485 = t482 * t484;
            let t487 = -t478 + t481 + f64x8::splat(0.93186) * t485;
            let t488 = t475 * t487;
            let t489 = t473 * t488;
            let t493 = (-t464 + t16 * t20 * t469 / f64x8::splat(4.0) - t207 * t489 / f64x8::splat(4.0)) * t350;
            let t494 = t493 * t353;
            let t495 = f64x8::splat(1.0) / t124;
            let t496 = t495 * t130;
            let t497 = t356 * t496;
            let t499 = f64x8::splat(0.005181783333333334) * t494 * t497;
            let t500 = t138 * t138;
            let t501 = f64x8::splat(1.0) / t500;
            let t502 = t501 * t482;
            let t504 = f64x8::splat(37.8469910464) * t501 + f64x8::splat(1.0);
            let t505 = f64x8::splat(1.0) / t504;
            let t506 = t484 * t505;
            let t508 = f64x8::splat(0.11929723702508031) * t502 * t506;
            let t509 = t144 * t131;
            let t512 = t145 * t475;
            let t514 = t509 * t485 / f64x8::splat(2.0) - t512 * t487;
            let t515 = f64x8::splat(1.0) / t145;
            let t516 = t514 * t515;
            let t518 = f64x8::splat(0.0009690227711544374) * t516 * t130;
            let t521 = t16 * t318 * t152 / f64x8::splat(12.0);
            let t522 = t468 * t151;
            let t526 = t150 * t150;
            let t527 = f64x8::splat(1.0) / t526;
            let t529 = -t478 + t481 + f64x8::splat(1.765105) * t485;
            let t530 = t527 * t529;
            let t531 = t473 * t530;
            let t535 = (-t521 + t16 * t20 * t522 / f64x8::splat(4.0) - t207 * t531 / f64x8::splat(4.0)) * t350;
            let t536 = t535 * t353;
            let t537 = t495 * t150;
            let t538 = t356 * t537;
            let t541 = t158 * t158;
            let t542 = f64x8::splat(1.0) / t541;
            let t543 = t542 * t482;
            let t545 = f64x8::splat(22.3816694236) * t542 + f64x8::splat(1.0);
            let t546 = f64x8::splat(1.0) / t545;
            let t547 = t484 * t546;
            let t550 = t163 * t151;
            let t553 = t164 * t527;
            let t555 = t550 * t485 / f64x8::splat(2.0) - t553 * t529;
            let t556 = f64x8::splat(1.0) / t164;
            let t557 = t555 * t556;
            let t563 = t499 - t508 + t518 + (f64x8::splat(0.002590891666666667) * t536 * t538 - f64x8::splat(0.12416647223360827) * t543 * t547 + f64x8::splat(0.0022478670955426118) * t557 * t150 - t499 + t508 - t518) * t76 * t80;
            let t567 = ((t120).select(f64x8::splat(0.0), t121 * t563 / f64x8::splat(2.0) + t460 * t171 / f64x8::splat(2.0)));
            let t568 = t567 * t204;
            let t569 = t15 * t317;
            let t573 = t211 * t211;
            let t574 = f64x8::splat(1.0) / t573;
            let t575 = t18 * t574;
            let t576 = t13 * t569;
            let t577 = t576 / f64x8::splat(12.0);
            let t578 = f64x8::splat(1.0) / t209;
            let t579 = t578 * t10;
            let t580 = t12 * t15;
            let t582 = t579 * t580 * t317;
            let t584 = -t577 - f64x8::splat(0.31062) * t582;
            let t590 = (-t13 * t569 * t212 / f64x8::splat(12.0) - t16 * t575 * t584 / f64x8::splat(4.0)) * t350 * t352;
            let t591 = t14 * t17;
            let t592 = t591 * t211;
            let t594 = f64x8::splat(0.010363566666666667) * t590 * t592;
            let t595 = t218 * t218;
            let t596 = f64x8::splat(1.0) / t595;
            let t598 = t596 * t578 * t10;
            let t600 = f64x8::splat(37.8469910464) * t596 + f64x8::splat(1.0);
            let t601 = f64x8::splat(1.0) / t600;
            let t605 = f64x8::splat(0.03976574567502677) * t598 * t580 * t317 * t601;
            let t606 = t224 * t212;
            let t607 = t606 * t578;
            let t610 = t225 * t574;
            let t612 = -t607 * t576 / f64x8::splat(6.0) - t610 * t584;
            let t613 = f64x8::splat(1.0) / t225;
            let t614 = t612 * t613;
            let t616 = f64x8::splat(0.0009690227711544374) * t614 * t211;
            let t620 = t232 * t232;
            let t621 = f64x8::splat(1.0) / t620;
            let t622 = t18 * t621;
            let t624 = -t577 - f64x8::splat(0.09425583333333333) * t582;
            let t630 = (-t13 * t569 * t233 / f64x8::splat(12.0) - t16 * t622 * t624 / f64x8::splat(4.0)) * t350 * t352;
            let t631 = t591 * t232;
            let t634 = t238 * t238;
            let t635 = f64x8::splat(1.0) / t634;
            let t637 = t635 * t578 * t10;
            let t639 = f64x8::splat(50.7386806551) * t635 + f64x8::splat(1.0);
            let t640 = f64x8::splat(1.0) / t639;
            let t645 = t243 * t233;
            let t646 = t645 * t578;
            let t649 = t244 * t621;
            let t651 = -t646 * t576 / f64x8::splat(6.0) - t649 * t624;
            let t652 = f64x8::splat(1.0) / t244;
            let t653 = t651 * t652;
            let t657 = t230 * (t630 * t631 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t637 * t580 * t317 * t640 + f64x8::splat(0.00041403379428206277) * t653 * t232);
            let t658 = t657 * t254;
            let t660 = t658 * t265 / f64x8::splat(24.0);
            let t663 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t313));
            let t666 = ((t119).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t122 * t459));
            let t667 = t663 + t666;
            let t668 = t249 * t667;
            let t671 = t256 * t2;
            let t672 = t671 * t260;
            let t673 = t259 * t3;
            let t674 = f64x8::splat(1.0) / t673;
            let t675 = t257 * t674;
            let t679 = t80 * (-f64x8::splat(4.0) * t672 + f64x8::splat(4.0) * t675) * t264;
            let t685 = t269 * t269;
            let t686 = f64x8::splat(1.0) / t685;
            let t687 = t18 * t686;
            let t689 = -t577 - f64x8::splat(0.5883683333333334) * t582;
            let t695 = (-t13 * t569 * t270 / f64x8::splat(12.0) - t16 * t687 * t689 / f64x8::splat(4.0)) * t350 * t352;
            let t696 = t591 * t269;
            let t699 = t276 * t276;
            let t700 = f64x8::splat(1.0) / t699;
            let t702 = t700 * t578 * t10;
            let t704 = f64x8::splat(22.3816694236) * t700 + f64x8::splat(1.0);
            let t705 = f64x8::splat(1.0) / t704;
            let t710 = t281 * t270;
            let t711 = t710 * t578;
            let t714 = t282 * t686;
            let t716 = -t711 * t576 / f64x8::splat(6.0) - t714 * t689;
            let t717 = f64x8::splat(1.0) / t282;
            let t718 = t716 * t717;
            let t721 = f64x8::splat(0.005181783333333334) * t695 * t696 + f64x8::splat(0.041388824077869424) * t702 * t580 * t317 * t705 + f64x8::splat(0.0022478670955426118) * t718 * t269 - t594 - t605 - t616;
            let t722 = t721 * t254;
            let t723 = t722 * t289;
            let t724 = t286 * t667;
            let t726 = t80 * t671;
            let t727 = t726 * t260;
            let t729 = f64x8::splat(4.0) * t287 * t727;
            let t730 = t288 * t674;
            let t732 = f64x8::splat(4.0) * t287 * t730;
            let t733 = t594 + t605 + t616 - t660 - t668 * t265 / f64x8::splat(24.0) - t255 * t679 / f64x8::splat(24.0) + t723 + t724 * t289 + t729 - t732 - t429 - t567;
            let t734 = t733 * t309;
            let t737 = t292 * t301;
            let t740 = t299 * t306;
            let t743 = t300 * t300;
            let t744 = f64x8::splat(1.0) / t743;
            let t745 = t304 * t744;
            let t748 = -f64x8::splat(0.01376408) * t434 * t296 + f64x8::splat(0.00017497128) * t737 * t434 + f64x8::splat(5.8717152e-07) * t740 * t434 - f64x8::splat(2.96462592e-09) * t745 * t434;
            let t749 = t291 * t748;
            let tvrho0 = t116 + t205 + t310 + t3 * (t430 + t458 + t568 + t734 + t749);
            acc_vrho_0 = tvrho0;
            let t752 = -t4 - t312;
            let t753 = ((t7).select(f64x8::splat(0.0), t752));
            let t757 = ((t7).select(f64x8::splat(0.0), -t322 * t752 / f64x8::splat(3.0)));
            let t758 = t757 * t32;
            let t763 = t16 * t20 * t757;
            let t764 = t763 / f64x8::splat(4.0);
            let t765 = -t340 + t763;
            let t766 = t339 * t765;
            let t768 = -t335 + t764 + f64x8::splat(0.93186) * t766;
            let t769 = t332 * t768;
            let t770 = t330 * t769;
            let t774 = (-t321 + t16 * t20 * t758 / f64x8::splat(4.0) - t207 * t770 / f64x8::splat(4.0)) * t350;
            let t775 = t774 * t353;
            let t777 = f64x8::splat(0.005181783333333334) * t775 * t359;
            let t778 = t765 * t367;
            let t780 = f64x8::splat(0.11929723702508031) * t364 * t778;
            let t784 = t371 * t766 / f64x8::splat(2.0) - t374 * t768;
            let t785 = t784 * t377;
            let t787 = f64x8::splat(0.0009690227711544374) * t785 * t31;
            let t788 = t757 * t52;
            let t793 = -t335 + t764 + f64x8::splat(1.765105) * t766;
            let t794 = t389 * t793;
            let t795 = t330 * t794;
            let t799 = (-t383 + t16 * t20 * t788 / f64x8::splat(4.0) - t207 * t795 / f64x8::splat(4.0)) * t350;
            let t800 = t799 * t353;
            let t803 = t765 * t408;
            let t809 = t412 * t766 / f64x8::splat(2.0) - t415 * t793;
            let t810 = t809 * t418;
            let t816 = t777 - t780 + t787 + (f64x8::splat(0.002590891666666667) * t800 * t400 - f64x8::splat(0.12416647223360827) * t405 * t803 + f64x8::splat(0.0022478670955426118) * t810 * t51 - t777 + t780 - t787) * t76 * t80;
            let t820 = ((t8).select(f64x8::splat(0.0), t753 * t82 / f64x8::splat(2.0) + t9 * t816 / f64x8::splat(2.0)));
            let t821 = t820 * t115;
            let t822 = -t752;
            let t823 = ((t119).select(f64x8::splat(0.0), t822));
            let t827 = ((t119).select(f64x8::splat(0.0), -t465 * t822 / f64x8::splat(3.0)));
            let t828 = t827 * t131;
            let t833 = t16 * t20 * t827;
            let t834 = t833 / f64x8::splat(4.0);
            let t835 = -t483 + t833;
            let t836 = t482 * t835;
            let t838 = -t478 + t834 + f64x8::splat(0.93186) * t836;
            let t839 = t475 * t838;
            let t840 = t473 * t839;
            let t844 = (-t464 + t16 * t20 * t828 / f64x8::splat(4.0) - t207 * t840 / f64x8::splat(4.0)) * t350;
            let t845 = t844 * t353;
            let t847 = f64x8::splat(0.005181783333333334) * t845 * t497;
            let t848 = t835 * t505;
            let t850 = f64x8::splat(0.11929723702508031) * t502 * t848;
            let t854 = t509 * t836 / f64x8::splat(2.0) - t512 * t838;
            let t855 = t854 * t515;
            let t857 = f64x8::splat(0.0009690227711544374) * t855 * t130;
            let t858 = t827 * t151;
            let t863 = -t478 + t834 + f64x8::splat(1.765105) * t836;
            let t864 = t527 * t863;
            let t865 = t473 * t864;
            let t869 = (-t521 + t16 * t20 * t858 / f64x8::splat(4.0) - t207 * t865 / f64x8::splat(4.0)) * t350;
            let t870 = t869 * t353;
            let t873 = t835 * t546;
            let t879 = t550 * t836 / f64x8::splat(2.0) - t553 * t863;
            let t880 = t879 * t556;
            let t886 = t847 - t850 + t857 + (f64x8::splat(0.002590891666666667) * t870 * t538 - f64x8::splat(0.12416647223360827) * t543 * t873 + f64x8::splat(0.0022478670955426118) * t880 * t150 - t847 + t850 - t857) * t76 * t80;
            let t890 = ((t120).select(f64x8::splat(0.0), t121 * t886 / f64x8::splat(2.0) + t823 * t171 / f64x8::splat(2.0)));
            let t891 = t890 * t204;
            let t892 = t175 * v_rho1;
            let t894 = f64x8::splat(1.0) / t177 / t892;
            let t895 = v_sigma2 * t894;
            let t898 = t187 * t175;
            let t900 = f64x8::splat(1.0) / t176 / t898;
            let t904 = t197 * v_rho1;
            let t905 = f64x8::splat(1.0) / t904;
            let t909 = t186 * t186;
            let t910 = t197 * t892;
            let t912 = f64x8::splat(1.0) / t177 / t910;
            let t914 = t192 * t192;
            let t915 = f64x8::splat(1.0) / t914;
            let t918 = -f64x8::splat(0.143424) * t895 * t183 + f64x8::splat(0.14618218666666666) * t186 * t900 * t193 - f64x8::splat(0.09002555733333334) * t196 * t905 * t201 + f64x8::splat(0.013305216) * t909 * t912 * t915;
            let t919 = t174 * t918;
            let t922 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t752));
            let t925 = ((t119).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t122 * t822));
            let t926 = t922 + t925;
            let t927 = t249 * t926;
            let t933 = t80 * (f64x8::splat(4.0) * t672 + f64x8::splat(4.0) * t675) * t264;
            let t936 = t286 * t926;
            let t938 = t594 + t605 + t616 - t660 - t927 * t265 / f64x8::splat(24.0) - t255 * t933 / f64x8::splat(24.0) + t723 + t936 * t289 - t729 - t732 - t820 - t890;
            let t939 = t938 * t309;
            let t948 = -f64x8::splat(0.01376408) * t895 * t296 + f64x8::splat(0.00017497128) * t737 * t895 + f64x8::splat(5.8717152e-07) * t740 * t895 - f64x8::splat(2.96462592e-09) * t745 * t895;
            let t949 = t291 * t948;
            let tvrho1 = t116 + t205 + t310 + t3 * (t821 + t891 + t919 + t939 + t949);
            acc_vrho_1 = tvrho1;
            let t960 = t108 * t86;
            let t962 = f64x8::splat(1.0) / t88 / t960;
            let t966 = f64x8::splat(0.053784) * t90 * t94 - f64x8::splat(0.05481832) * v_sigma0 * t101 * t104 + f64x8::splat(0.033759584) * t97 * t109 * t112 - f64x8::splat(0.004989456) * t107 * t962 * t454;
            let t967 = t85 * t966;
            let t976 = f64x8::splat(0.00516153) * t90 * t296 - f64x8::splat(6.561423e-05) * t737 * t90 - f64x8::splat(2.2018932e-07) * t740 * t90 + f64x8::splat(1.11173472e-09) * t745 * t90;
            let t977 = t291 * t976;
            let tvsigma0 = t3 * (t967 + t977);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t987 = t197 * t175;
            let t989 = f64x8::splat(1.0) / t177 / t987;
            let t993 = f64x8::splat(0.053784) * t179 * t183 - f64x8::splat(0.05481832) * v_sigma2 * t190 * t193 + f64x8::splat(0.033759584) * t186 * t198 * t201 - f64x8::splat(0.004989456) * t196 * t989 * t915;
            let t994 = t174 * t993;
            let t1003 = f64x8::splat(0.00516153) * t179 * t296 - f64x8::splat(6.561423e-05) * t737 * t179 - f64x8::splat(2.2018932e-07) * t740 * t179 + f64x8::splat(1.11173472e-09) * t745 * t179;
            let t1004 = t291 * t1003;
            let tvsigma2 = t3 * (t994 + t1004);
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
