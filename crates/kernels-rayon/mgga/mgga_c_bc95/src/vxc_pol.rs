//! MGGA_C_BC95 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`
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
pub fn mgga_c_bc95_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_copp = f64x8::splat(param_copp);
    let param_css = f64x8::splat(param_css);
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
            let t3 = v_rho0 - v_rho1;
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = f64x8::splat(1.0) + t6;
            let t8 = (t7).simd_le(zeta_threshold);
            let t9 = ((v_rho0).simd_le(dens_threshold)) | (t8);
            let t10 = ((t8).select(zeta_threshold, t7));
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = t14 * t16;
            let t18 = (simd::cbrt(t4));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = (simd::cbrt(t7));
            let t26 = ((t8).select(t23, f64x8::splat(1.0) / t24));
            let t28 = t17 * t21 * t26;
            let t30 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t28;
            let t31 = ((t28).sqrt());
            let t34 = ((t28) * (t28).sqrt());
            let t36 = t11 * t11;
            let t37 = t13 * t13;
            let t38 = t36 * t37;
            let t39 = t38 * t15;
            let t40 = t18 * t18;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t20 * t20;
            let t43 = t41 * t42;
            let t44 = t26 * t26;
            let t46 = t39 * t43 * t44;
            let t48 = f64x8::splat(3.79785) * t31 + f64x8::splat(0.8969) * t28 + f64x8::splat(0.204775) * t34 + f64x8::splat(0.123235) * t46;
            let t51 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t48;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0621814) * t30 * t52;
            let t56 = t22 * zeta_threshold;
            let t58 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(2.0) * t20));
            let t60 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(0.0)));
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t20 - f64x8::splat(2.0));
            let t65 = (t58 + t60 - f64x8::splat(2.0)) * t64;
            let t67 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t28;
            let t72 = f64x8::splat(7.05945) * t31 + f64x8::splat(1.549425) * t28 + f64x8::splat(0.420775) * t34 + f64x8::splat(0.1562925) * t46;
            let t75 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t72;
            let t76 = (simd::ln(t75));
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t28;
            let t85 = f64x8::splat(5.1785) * t31 + f64x8::splat(0.905775) * t28 + f64x8::splat(0.1100325) * t34 + f64x8::splat(0.1241775) * t46;
            let t88 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t85;
            let t89 = (simd::ln(t88));
            let t90 = t80 * t89;
            let t96 = -t54 + t65 * (-f64x8::splat(0.0310907) * t67 * t76 + t54 - f64x8::splat(0.0197516734986138) * t90) + f64x8::splat(0.0197516734986138) * t65 * t90;
            let t99 = ((t9).select(f64x8::splat(0.0), t10 * t96 / f64x8::splat(2.0)));
            let t100 = t99 * v_tau0;
            let t101 = (simd::cbrt(v_rho0));
            let t102 = t101 * t101;
            let t104 = f64x8::splat(1.0) / t102 / v_rho0;
            let t108 = f64x8::splat(1.0) / v_tau0;
            let t111 = f64x8::splat(1.0) - v_sigma0 / v_rho0 * t108 / f64x8::splat(8.0);
            let t112 = f64x8::splat(M_CBRT6);
            let t113 = t111 * t112;
            let t114 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t115 = (simd::cbrt(t114));
            let t116 = t115 * t115;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = param_css * v_sigma0;
            let t119 = v_rho0 * v_rho0;
            let t121 = f64x8::splat(1.0) / t102 / t119;
            let t123 = t118 * t121 + f64x8::splat(1.0);
            let t124 = t123 * t123;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t117 * t125;
            let t127 = t113 * t126;
            let t129 = f64x8::splat(5.0) / f64x8::splat(9.0) * t100 * t104 * t127;
            let t131 = f64x8::splat(1.0) - t6;
            let t132 = (t131).simd_le(zeta_threshold);
            let t133 = ((v_rho1).simd_le(dens_threshold)) | (t132);
            let t134 = ((t132).select(zeta_threshold, t131));
            let t135 = (simd::cbrt(t131));
            let t137 = ((t132).select(t23, f64x8::splat(1.0) / t135));
            let t139 = t17 * t21 * t137;
            let t141 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t139;
            let t142 = ((t139).sqrt());
            let t145 = ((t139) * (t139).sqrt());
            let t147 = t137 * t137;
            let t149 = t39 * t43 * t147;
            let t151 = f64x8::splat(3.79785) * t142 + f64x8::splat(0.8969) * t139 + f64x8::splat(0.204775) * t145 + f64x8::splat(0.123235) * t149;
            let t154 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t151;
            let t155 = (simd::ln(t154));
            let t157 = f64x8::splat(0.0621814) * t141 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t139;
            let t164 = f64x8::splat(7.05945) * t142 + f64x8::splat(1.549425) * t139 + f64x8::splat(0.420775) * t145 + f64x8::splat(0.1562925) * t149;
            let t167 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t164;
            let t168 = (simd::ln(t167));
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t139;
            let t177 = f64x8::splat(5.1785) * t142 + f64x8::splat(0.905775) * t139 + f64x8::splat(0.1100325) * t145 + f64x8::splat(0.1241775) * t149;
            let t180 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t177;
            let t181 = (simd::ln(t180));
            let t182 = t172 * t181;
            let t188 = -t157 + t65 * (-f64x8::splat(0.0310907) * t159 * t168 + t157 - f64x8::splat(0.0197516734986138) * t182) + f64x8::splat(0.0197516734986138) * t65 * t182;
            let t191 = ((t133).select(f64x8::splat(0.0), t134 * t188 / f64x8::splat(2.0)));
            let t192 = t191 * v_tau1;
            let t193 = (simd::cbrt(v_rho1));
            let t194 = t193 * t193;
            let t196 = f64x8::splat(1.0) / t194 / v_rho1;
            let t200 = f64x8::splat(1.0) / v_tau1;
            let t203 = f64x8::splat(1.0) - v_sigma2 / v_rho1 * t200 / f64x8::splat(8.0);
            let t204 = t203 * t112;
            let t205 = param_css * v_sigma2;
            let t206 = v_rho1 * v_rho1;
            let t208 = f64x8::splat(1.0) / t194 / t206;
            let t210 = t205 * t208 + f64x8::splat(1.0);
            let t211 = t210 * t210;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = t117 * t212;
            let t214 = t204 * t213;
            let t216 = f64x8::splat(5.0) / f64x8::splat(9.0) * t192 * t196 * t214;
            let t218 = t14 * t16 * t19;
            let t220 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t218;
            let t221 = ((t218).sqrt());
            let t224 = ((t218) * (t218).sqrt());
            let t227 = t38 * t15 * t41;
            let t229 = f64x8::splat(3.79785) * t221 + f64x8::splat(0.8969) * t218 + f64x8::splat(0.204775) * t224 + f64x8::splat(0.123235) * t227;
            let t232 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t229;
            let t233 = (simd::ln(t232));
            let t235 = f64x8::splat(0.0621814) * t220 * t233;
            let t236 = t3 * t3;
            let t237 = t236 * t236;
            let t238 = t4 * t4;
            let t239 = t238 * t238;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t237 * t240;
            let t242 = t24 * t7;
            let t243 = ((t8).select(t56, t242));
            let t244 = t135 * t131;
            let t245 = ((t132).select(t56, t244));
            let t246 = t243 + t245 - f64x8::splat(2.0);
            let t247 = t246 * t64;
            let t249 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t218;
            let t254 = f64x8::splat(7.05945) * t221 + f64x8::splat(1.549425) * t218 + f64x8::splat(0.420775) * t224 + f64x8::splat(0.1562925) * t227;
            let t257 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t254;
            let t258 = (simd::ln(t257));
            let t262 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t218;
            let t267 = f64x8::splat(5.1785) * t221 + f64x8::splat(0.905775) * t218 + f64x8::splat(0.1100325) * t224 + f64x8::splat(0.1241775) * t227;
            let t270 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t267;
            let t271 = (simd::ln(t270));
            let t272 = t262 * t271;
            let t274 = -f64x8::splat(0.0310907) * t249 * t258 + t235 - f64x8::splat(0.0197516734986138) * t272;
            let t275 = t247 * t274;
            let t279 = -t235 + t241 * t275 + f64x8::splat(0.0197516734986138) * t247 * t272 - t99 - t191;
            let t284 = f64x8::splat(1.0) + param_copp * (v_sigma0 * t121 + v_sigma2 * t208);
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t279 * t285;
            let tzk0 = t129 + t216 + t286;
            acc_zk = tzk0;
            let t287 = f64x8::splat(1.0) / t238;
            let t288 = t3 * t287;
            let t289 = t5 - t288;
            let t290 = ((t8).select(f64x8::splat(0.0), t289));
            let t293 = f64x8::splat(1.0) / t18 / t4;
            let t294 = t293 * t20;
            let t296 = t17 * t294 * t26;
            let t297 = f64x8::splat(0.017808333333333332) * t296;
            let t298 = f64x8::splat(1.0) / t242;
            let t301 = ((t8).select(f64x8::splat(0.0), -t298 * t289 / f64x8::splat(3.0)));
            let t303 = t17 * t21 * t301;
            let t305 = -t297 + f64x8::splat(0.053425) * t303;
            let t307 = f64x8::splat(0.0621814) * t305 * t52;
            let t308 = t48 * t48;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t30 * t309;
            let t311 = f64x8::splat(1.0) / t31;
            let t312 = t296 / f64x8::splat(3.0);
            let t313 = -t312 + t303;
            let t314 = t311 * t313;
            let t316 = f64x8::splat(0.29896666666666666) * t296;
            let t318 = ((t28).sqrt());
            let t319 = t318 * t313;
            let t322 = f64x8::splat(1.0) / t40 / t4;
            let t323 = t322 * t42;
            let t325 = t39 * t323 * t44;
            let t326 = f64x8::splat(0.08215666666666667) * t325;
            let t327 = t26 * t301;
            let t329 = t39 * t43 * t327;
            let t331 = f64x8::splat(1.898925) * t314 - t316 + f64x8::splat(0.8969) * t303 + f64x8::splat(0.3071625) * t319 - t326 + f64x8::splat(0.24647) * t329;
            let t332 = f64x8::splat(1.0) / t51;
            let t333 = t331 * t332;
            let t335 = f64x8::splat(1.0) * t310 * t333;
            let t336 = f64x8::splat(0.017123333333333334) * t296;
            let t338 = -t336 + f64x8::splat(0.05137) * t303;
            let t341 = t72 * t72;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t67 * t342;
            let t345 = f64x8::splat(0.516475) * t296;
            let t348 = f64x8::splat(0.104195) * t325;
            let t350 = f64x8::splat(3.529725) * t314 - t345 + f64x8::splat(1.549425) * t303 + f64x8::splat(0.6311625) * t319 - t348 + f64x8::splat(0.312585) * t329;
            let t351 = f64x8::splat(1.0) / t75;
            let t352 = t350 * t351;
            let t355 = f64x8::splat(0.009270833333333334) * t296;
            let t357 = -t355 + f64x8::splat(0.0278125) * t303;
            let t358 = t357 * t89;
            let t360 = t85 * t85;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t80 * t361;
            let t364 = f64x8::splat(0.301925) * t296;
            let t367 = f64x8::splat(0.082785) * t325;
            let t369 = f64x8::splat(2.58925) * t314 - t364 + f64x8::splat(0.905775) * t303 + f64x8::splat(0.16504875) * t319 - t367 + f64x8::splat(0.248355) * t329;
            let t370 = f64x8::splat(1.0) / t88;
            let t371 = t369 * t370;
            let t378 = t65 * t80;
            let t380 = t361 * t369 * t370;
            let t383 = -t307 + t335 + t65 * (-f64x8::splat(0.0310907) * t338 * t76 + f64x8::splat(1.0) * t343 * t352 + t307 - t335 - f64x8::splat(0.0197516734986138) * t358 + f64x8::splat(0.5848223622634646) * t362 * t371) + f64x8::splat(0.0197516734986138) * t65 * t358 - f64x8::splat(0.5848223622634646) * t378 * t380;
            let t387 = ((t9).select(f64x8::splat(0.0), t10 * t383 / f64x8::splat(2.0) + t290 * t96 / f64x8::splat(2.0)));
            let t388 = t387 * v_tau0;
            let t390 = t388 * t104 * t127;
            let t391 = f64x8::splat(5.0) / f64x8::splat(9.0) * t390;
            let t393 = t100 * t121 * t127;
            let t394 = f64x8::splat(25.0) / f64x8::splat(27.0) * t393;
            let t395 = t119 * v_rho0;
            let t397 = f64x8::splat(1.0) / t102 / t395;
            let t398 = t99 * t397;
            let t399 = t398 * v_sigma0;
            let t400 = t112 * t117;
            let t401 = t400 * t125;
            let t402 = t399 * t401;
            let t403 = f64x8::splat(5.0) / f64x8::splat(72.0) * t402;
            let t404 = t119 * t119;
            let t405 = t404 * v_rho0;
            let t407 = f64x8::splat(1.0) / t101 / t405;
            let t408 = t407 * t111;
            let t409 = t100 * t408;
            let t411 = f64x8::splat(1.0) / t124 / t123;
            let t412 = t411 * param_css;
            let t414 = t400 * t412 * v_sigma0;
            let t415 = t409 * t414;
            let t416 = f64x8::splat(80.0) / f64x8::splat(27.0) * t415;
            let t417 = -t289;
            let t418 = ((t132).select(f64x8::splat(0.0), t417));
            let t421 = t17 * t294 * t137;
            let t422 = f64x8::splat(0.017808333333333332) * t421;
            let t423 = f64x8::splat(1.0) / t244;
            let t426 = ((t132).select(f64x8::splat(0.0), -t423 * t417 / f64x8::splat(3.0)));
            let t428 = t17 * t21 * t426;
            let t430 = -t422 + f64x8::splat(0.053425) * t428;
            let t432 = f64x8::splat(0.0621814) * t430 * t155;
            let t433 = t151 * t151;
            let t434 = f64x8::splat(1.0) / t433;
            let t435 = t141 * t434;
            let t436 = f64x8::splat(1.0) / t142;
            let t437 = t421 / f64x8::splat(3.0);
            let t438 = -t437 + t428;
            let t439 = t436 * t438;
            let t441 = f64x8::splat(0.29896666666666666) * t421;
            let t443 = ((t139).sqrt());
            let t444 = t443 * t438;
            let t447 = t39 * t323 * t147;
            let t448 = f64x8::splat(0.08215666666666667) * t447;
            let t449 = t137 * t426;
            let t451 = t39 * t43 * t449;
            let t453 = f64x8::splat(1.898925) * t439 - t441 + f64x8::splat(0.8969) * t428 + f64x8::splat(0.3071625) * t444 - t448 + f64x8::splat(0.24647) * t451;
            let t454 = f64x8::splat(1.0) / t154;
            let t455 = t453 * t454;
            let t457 = f64x8::splat(1.0) * t435 * t455;
            let t458 = f64x8::splat(0.017123333333333334) * t421;
            let t460 = -t458 + f64x8::splat(0.05137) * t428;
            let t463 = t164 * t164;
            let t464 = f64x8::splat(1.0) / t463;
            let t465 = t159 * t464;
            let t467 = f64x8::splat(0.516475) * t421;
            let t470 = f64x8::splat(0.104195) * t447;
            let t472 = f64x8::splat(3.529725) * t439 - t467 + f64x8::splat(1.549425) * t428 + f64x8::splat(0.6311625) * t444 - t470 + f64x8::splat(0.312585) * t451;
            let t473 = f64x8::splat(1.0) / t167;
            let t474 = t472 * t473;
            let t477 = f64x8::splat(0.009270833333333334) * t421;
            let t479 = -t477 + f64x8::splat(0.0278125) * t428;
            let t480 = t479 * t181;
            let t482 = t177 * t177;
            let t483 = f64x8::splat(1.0) / t482;
            let t484 = t172 * t483;
            let t486 = f64x8::splat(0.301925) * t421;
            let t489 = f64x8::splat(0.082785) * t447;
            let t491 = f64x8::splat(2.58925) * t439 - t486 + f64x8::splat(0.905775) * t428 + f64x8::splat(0.16504875) * t444 - t489 + f64x8::splat(0.248355) * t451;
            let t492 = f64x8::splat(1.0) / t180;
            let t493 = t491 * t492;
            let t500 = t65 * t172;
            let t502 = t483 * t491 * t492;
            let t505 = -t432 + t457 + t65 * (-f64x8::splat(0.0310907) * t460 * t168 + f64x8::splat(1.0) * t465 * t474 + t432 - t457 - f64x8::splat(0.0197516734986138) * t480 + f64x8::splat(0.5848223622634646) * t484 * t493) + f64x8::splat(0.0197516734986138) * t65 * t480 - f64x8::splat(0.5848223622634646) * t500 * t502;
            let t509 = ((t133).select(f64x8::splat(0.0), t134 * t505 / f64x8::splat(2.0) + t418 * t188 / f64x8::splat(2.0)));
            let t510 = t509 * v_tau1;
            let t512 = t510 * t196 * t214;
            let t513 = f64x8::splat(5.0) / f64x8::splat(9.0) * t512;
            let t514 = t16 * t293;
            let t517 = f64x8::splat(0.0011073470983333333) * t14 * t514 * t233;
            let t518 = t229 * t229;
            let t519 = f64x8::splat(1.0) / t518;
            let t520 = t220 * t519;
            let t522 = f64x8::splat(1.0) / t221 * t11;
            let t523 = t13 * t16;
            let t524 = t523 * t293;
            let t525 = t522 * t524;
            let t527 = t14 * t514;
            let t529 = ((t218).sqrt());
            let t530 = t529 * t11;
            let t531 = t530 * t524;
            let t534 = t38 * t15 * t322;
            let t536 = -f64x8::splat(0.632975) * t525 - f64x8::splat(0.29896666666666666) * t527 - f64x8::splat(0.1023875) * t531 - f64x8::splat(0.08215666666666667) * t534;
            let t537 = f64x8::splat(1.0) / t232;
            let t538 = t536 * t537;
            let t540 = f64x8::splat(1.0) * t520 * t538;
            let t541 = t236 * t3;
            let t542 = t541 * t240;
            let t544 = f64x8::splat(4.0) * t542 * t275;
            let t545 = t239 * t4;
            let t546 = f64x8::splat(1.0) / t545;
            let t547 = t237 * t546;
            let t549 = f64x8::splat(4.0) * t547 * t275;
            let t552 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t289));
            let t555 = ((t132).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t135 * t417));
            let t557 = (t552 + t555) * t64;
            let t558 = t557 * t274;
            let t563 = t254 * t254;
            let t564 = f64x8::splat(1.0) / t563;
            let t565 = t249 * t564;
            let t570 = -f64x8::splat(1.176575) * t525 - f64x8::splat(0.516475) * t527 - f64x8::splat(0.2103875) * t531 - f64x8::splat(0.104195) * t534;
            let t571 = f64x8::splat(1.0) / t257;
            let t572 = t570 * t571;
            let t578 = t267 * t267;
            let t579 = f64x8::splat(1.0) / t578;
            let t580 = t262 * t579;
            let t585 = -f64x8::splat(0.8630833333333333) * t525 - f64x8::splat(0.301925) * t527 - f64x8::splat(0.05501625) * t531 - f64x8::splat(0.082785) * t534;
            let t586 = f64x8::splat(1.0) / t270;
            let t587 = t585 * t586;
            let t590 = f64x8::splat(0.0005323764196666666) * t14 * t514 * t258 + f64x8::splat(1.0) * t565 * t572 - t517 - t540 + f64x8::splat(0.00018311447306006544) * t14 * t514 * t271 + f64x8::splat(0.5848223622634646) * t580 * t587;
            let t591 = t247 * t590;
            let t592 = t241 * t591;
            let t595 = t247 * t11;
            let t597 = t523 * t293 * t271;
            let t599 = f64x8::splat(0.00018311447306006544) * t595 * t597;
            let t600 = t247 * t262;
            let t602 = t579 * t585 * t586;
            let t604 = f64x8::splat(0.5848223622634646) * t600 * t602;
            let t605 = t517 + t540 + t544 - t549 + t241 * t558 + t592 + f64x8::splat(0.0197516734986138) * t557 * t272 - t599 - t604 - t387 - t509;
            let t606 = t605 * t285;
            let t607 = t284 * t284;
            let t608 = f64x8::splat(1.0) / t607;
            let t609 = t279 * t608;
            let t610 = param_copp * v_sigma0;
            let t611 = t610 * t397;
            let t612 = t609 * t611;
            let t613 = f64x8::splat(8.0) / f64x8::splat(3.0) * t612;
            let tvrho0 = t129 + t216 + t286 + t4 * (t391 - t394 + t403 + t416 + t513 + t606 + t613);
            acc_vrho_0 = tvrho0;
            let t616 = -t5 - t288;
            let t617 = ((t8).select(f64x8::splat(0.0), t616));
            let t621 = ((t8).select(f64x8::splat(0.0), -t298 * t616 / f64x8::splat(3.0)));
            let t623 = t17 * t21 * t621;
            let t625 = -t297 + f64x8::splat(0.053425) * t623;
            let t627 = f64x8::splat(0.0621814) * t625 * t52;
            let t628 = -t312 + t623;
            let t629 = t311 * t628;
            let t632 = t318 * t628;
            let t634 = t26 * t621;
            let t636 = t39 * t43 * t634;
            let t638 = f64x8::splat(1.898925) * t629 - t316 + f64x8::splat(0.8969) * t623 + f64x8::splat(0.3071625) * t632 - t326 + f64x8::splat(0.24647) * t636;
            let t639 = t638 * t332;
            let t641 = f64x8::splat(1.0) * t310 * t639;
            let t643 = -t336 + f64x8::splat(0.05137) * t623;
            let t650 = f64x8::splat(3.529725) * t629 - t345 + f64x8::splat(1.549425) * t623 + f64x8::splat(0.6311625) * t632 - t348 + f64x8::splat(0.312585) * t636;
            let t651 = t650 * t351;
            let t655 = -t355 + f64x8::splat(0.0278125) * t623;
            let t656 = t655 * t89;
            let t662 = f64x8::splat(2.58925) * t629 - t364 + f64x8::splat(0.905775) * t623 + f64x8::splat(0.16504875) * t632 - t367 + f64x8::splat(0.248355) * t636;
            let t663 = t662 * t370;
            let t671 = t361 * t662 * t370;
            let t674 = -t627 + t641 + t65 * (-f64x8::splat(0.0310907) * t643 * t76 + f64x8::splat(1.0) * t343 * t651 + t627 - t641 - f64x8::splat(0.0197516734986138) * t656 + f64x8::splat(0.5848223622634646) * t362 * t663) + f64x8::splat(0.0197516734986138) * t65 * t656 - f64x8::splat(0.5848223622634646) * t378 * t671;
            let t678 = ((t9).select(f64x8::splat(0.0), t10 * t674 / f64x8::splat(2.0) + t617 * t96 / f64x8::splat(2.0)));
            let t679 = t678 * v_tau0;
            let t681 = t679 * t104 * t127;
            let t682 = f64x8::splat(5.0) / f64x8::splat(9.0) * t681;
            let t683 = -t616;
            let t684 = ((t132).select(f64x8::splat(0.0), t683));
            let t688 = ((t132).select(f64x8::splat(0.0), -t423 * t683 / f64x8::splat(3.0)));
            let t690 = t17 * t21 * t688;
            let t692 = -t422 + f64x8::splat(0.053425) * t690;
            let t694 = f64x8::splat(0.0621814) * t692 * t155;
            let t695 = -t437 + t690;
            let t696 = t436 * t695;
            let t699 = t443 * t695;
            let t701 = t137 * t688;
            let t703 = t39 * t43 * t701;
            let t705 = f64x8::splat(1.898925) * t696 - t441 + f64x8::splat(0.8969) * t690 + f64x8::splat(0.3071625) * t699 - t448 + f64x8::splat(0.24647) * t703;
            let t706 = t705 * t454;
            let t708 = f64x8::splat(1.0) * t435 * t706;
            let t710 = -t458 + f64x8::splat(0.05137) * t690;
            let t717 = f64x8::splat(3.529725) * t696 - t467 + f64x8::splat(1.549425) * t690 + f64x8::splat(0.6311625) * t699 - t470 + f64x8::splat(0.312585) * t703;
            let t718 = t717 * t473;
            let t722 = -t477 + f64x8::splat(0.0278125) * t690;
            let t723 = t722 * t181;
            let t729 = f64x8::splat(2.58925) * t696 - t486 + f64x8::splat(0.905775) * t690 + f64x8::splat(0.16504875) * t699 - t489 + f64x8::splat(0.248355) * t703;
            let t730 = t729 * t492;
            let t738 = t483 * t729 * t492;
            let t741 = -t694 + t708 + t65 * (-f64x8::splat(0.0310907) * t710 * t168 + f64x8::splat(1.0) * t465 * t718 + t694 - t708 - f64x8::splat(0.0197516734986138) * t723 + f64x8::splat(0.5848223622634646) * t484 * t730) + f64x8::splat(0.0197516734986138) * t65 * t723 - f64x8::splat(0.5848223622634646) * t500 * t738;
            let t745 = ((t133).select(f64x8::splat(0.0), t134 * t741 / f64x8::splat(2.0) + t684 * t188 / f64x8::splat(2.0)));
            let t746 = t745 * v_tau1;
            let t748 = t746 * t196 * t214;
            let t749 = f64x8::splat(5.0) / f64x8::splat(9.0) * t748;
            let t751 = t192 * t208 * t214;
            let t752 = f64x8::splat(25.0) / f64x8::splat(27.0) * t751;
            let t753 = t206 * v_rho1;
            let t755 = f64x8::splat(1.0) / t194 / t753;
            let t756 = t191 * t755;
            let t757 = t756 * v_sigma2;
            let t758 = t400 * t212;
            let t759 = t757 * t758;
            let t760 = f64x8::splat(5.0) / f64x8::splat(72.0) * t759;
            let t761 = t206 * t206;
            let t762 = t761 * v_rho1;
            let t764 = f64x8::splat(1.0) / t193 / t762;
            let t765 = t764 * t203;
            let t766 = t192 * t765;
            let t768 = f64x8::splat(1.0) / t211 / t210;
            let t769 = t768 * param_css;
            let t771 = t400 * t769 * v_sigma2;
            let t772 = t766 * t771;
            let t773 = f64x8::splat(80.0) / f64x8::splat(27.0) * t772;
            let t776 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t616));
            let t779 = ((t132).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t135 * t683));
            let t781 = (t776 + t779) * t64;
            let t782 = t781 * t274;
            let t786 = t517 + t540 - t544 - t549 + t241 * t782 + t592 + f64x8::splat(0.0197516734986138) * t781 * t272 - t599 - t604 - t678 - t745;
            let t787 = t786 * t285;
            let t788 = param_copp * v_sigma2;
            let t789 = t788 * t755;
            let t790 = t609 * t789;
            let t791 = f64x8::splat(8.0) / f64x8::splat(3.0) * t790;
            let tvrho1 = t129 + t216 + t286 + t4 * (t682 + t749 - t752 + t760 + t773 + t787 + t791);
            acc_vrho_1 = tvrho1;
            let t794 = t99 * t121;
            let t796 = f64x8::splat(5.0) / f64x8::splat(72.0) * t794 * t401;
            let t798 = f64x8::splat(1.0) / t101 / t404;
            let t799 = t798 * t111;
            let t801 = t400 * t412;
            let t803 = f64x8::splat(10.0) / f64x8::splat(9.0) * t100 * t799 * t801;
            let t804 = param_copp * t121;
            let t805 = t609 * t804;
            let tvsigma0 = t4 * (-t796 - t803 - t805);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t807 = t191 * t208;
            let t809 = f64x8::splat(5.0) / f64x8::splat(72.0) * t807 * t758;
            let t811 = f64x8::splat(1.0) / t193 / t761;
            let t812 = t811 * t203;
            let t814 = t400 * t769;
            let t816 = f64x8::splat(10.0) / f64x8::splat(9.0) * t192 * t812 * t814;
            let t817 = param_copp * t208;
            let t818 = t609 * t817;
            let tvsigma2 = t4 * (-t809 - t816 - t818);
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t823 = f64x8::splat(5.0) / f64x8::splat(9.0) * t99 * t104 * t111 * t401;
            let t824 = t99 * t108;
            let t827 = v_sigma0 * t112 * t126;
            let t829 = f64x8::splat(5.0) / f64x8::splat(72.0) * t824 * t121 * t827;
            let tvtau0 = t4 * (t823 + t829);
            acc_vtau_0 = tvtau0;
            let t834 = f64x8::splat(5.0) / f64x8::splat(9.0) * t191 * t196 * t203 * t758;
            let t835 = t191 * t200;
            let t838 = v_sigma2 * t112 * t213;
            let t840 = f64x8::splat(5.0) / f64x8::splat(72.0) * t835 * t208 * t838;
            let tvtau1 = t4 * (t834 + t840);
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
