//! MGGA_C_RSCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`
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
pub fn mgga_c_rscan_vxc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t24 = f64x8::splat(1.0) / t23;
            let t26 = t22 * t6 * t24;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t127 = t9 * t38;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t126 * t129 * t130 * t134;
            let t139 = ((t138).sqrt().sqrt());
            let t141 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139;
            let t144 = f64x8::splat(1.0) + f64x8::splat(1.0) * t141 * t120;
            let t145 = (simd::ln(t144));
            let t147 = t97 * t106 * t145;
            let t148 = t39 * t8;
            let t149 = (simd::cbrt(v_rho0));
            let t150 = t149 * t149;
            let t152 = f64x8::splat(1.0) / t150 / v_rho0;
            let t153 = v_tau0 * t152;
            let t154 = t44 / f64x8::splat(2.0);
            let t155 = (simd::cbrt(t154));
            let t156 = t155 * t155;
            let t157 = t156 * t154;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / v_rho1;
            let t163 = v_tau1 * t162;
            let t164 = t51 / f64x8::splat(2.0);
            let t165 = (simd::cbrt(t164));
            let t166 = t165 * t165;
            let t167 = t166 * t164;
            let t169 = t23 * t38;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = t153 * t157 + t163 * t167 - t124 * t170 / f64x8::splat(8.0);
            let t174 = (f64x8::splat(0.0)).simd_lt(t173);
            let t175 = ((t174).select(t173, f64x8::splat(0.0)));
            let t176 = t175 * t175;
            let t177 = t176 * t175;
            let t178 = t148 * t177;
            let t179 = f64x8::splat(M_CBRT6);
            let t180 = t179 * t179;
            let t181 = (simd::cbrt(t95));
            let t182 = t181 * t181;
            let t183 = t180 * t182;
            let t184 = t23 * t8;
            let t187 = t57 * t57;
            let t189 = f64x8::splat(3.0) / f64x8::splat(10.0) * t183 * t184 + f64x8::splat(0.0001) * t187;
            let t190 = t189 * t189;
            let t191 = t190 * t189;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t157 + t167;
            let t194 = t193 * t193;
            let t195 = t194 * t193;
            let t196 = f64x8::splat(1.0) / t195;
            let t197 = t192 * t196;
            let t198 = t38 * t8;
            let t199 = t9 * t198;
            let t200 = t199 * t176;
            let t201 = f64x8::splat(1.0) / t190;
            let t202 = f64x8::splat(1.0) / t194;
            let t203 = t201 * t202;
            let t205 = t200 * t203 + f64x8::splat(0.001);
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t197 * t206;
            let t208 = t178 * t207;
            let t209 = (t208).simd_le(f64x8::splat(2.5));
            let t210 = (f64x8::splat(2.5)).simd_lt(t208);
            let t211 = ((t210).select(f64x8::splat(2.5), t208));
            let t213 = t211 * t211;
            let t215 = t213 * t211;
            let t217 = t213 * t213;
            let t219 = t217 * t211;
            let t221 = t217 * t213;
            let t226 = ((t210).select(t208, f64x8::splat(2.5)));
            let t227 = f64x8::splat(1.0) - t226;
            let t230 = (simd::exp(f64x8::splat(1.5) / t227));
            let t232 = ((t209).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t211 - f64x8::splat(0.4352) * t213 - f64x8::splat(1.535685604549) * t215 + f64x8::splat(3.061560252175) * t217 - f64x8::splat(1.915710236206) * t219 + f64x8::splat(0.516884468372) * t221 - f64x8::splat(0.051848879792) * t217 * t215, -f64x8::splat(0.7) * t230));
            let t235 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + f64x8::splat(0.03138525) * t12;
            let t236 = f64x8::splat(1.0) / t235;
            let t239 = (simd::exp(f64x8::splat(1.0) * t236));
            let t240 = t239 - f64x8::splat(1.0);
            let t241 = f64x8::splat(1.0) / t182;
            let t242 = t179 * t241;
            let t243 = t187 * t124;
            let t247 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t242 * t243 * t170;
            let t248 = ((t247).sqrt().sqrt());
            let t250 = f64x8::splat(1.0) - f64x8::splat(1.0) / t248;
            let t252 = t240 * t250 + f64x8::splat(1.0);
            let t253 = (simd::ln(t252));
            let t255 = -f64x8::splat(0.0285764) * t236 + f64x8::splat(0.0285764) * t253;
            let t259 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t260 = t255 * t259;
            let t261 = t37 * t37;
            let t262 = t261 * t37;
            let t263 = t39 * t39;
            let t264 = t263 * t39;
            let t265 = f64x8::splat(1.0) / t264;
            let t267 = -t262 * t265 + f64x8::splat(1.0);
            let t269 = t260 * t267 - t147 + t34 - t90 - t92;
            let t270 = t232 * t269;
            let tzk0 = -t34 + t90 + t92 + t147 + t270;
            acc_zk = tzk0;
            let t271 = t9 * t8;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t7 * t272;
            let t275 = t5 * t273 * t32;
            let t276 = f64x8::splat(0.0011073470983333333) * t275;
            let t277 = t28 * t28;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t14 * t278;
            let t281 = f64x8::splat(1.0) / t15 * t2;
            let t282 = t4 * t7;
            let t283 = t282 * t272;
            let t284 = t281 * t283;
            let t286 = t5 * t273;
            let t288 = ((t12).sqrt());
            let t289 = t288 * t2;
            let t290 = t289 * t283;
            let t294 = t22 * t6 / t184;
            let t296 = -f64x8::splat(0.632975) * t284 - f64x8::splat(0.29896666666666666) * t286 - f64x8::splat(0.1023875) * t290 - f64x8::splat(0.08215666666666667) * t294;
            let t297 = f64x8::splat(1.0) / t31;
            let t298 = t296 * t297;
            let t299 = t279 * t298;
            let t300 = f64x8::splat(1.0) * t299;
            let t301 = t36 * t35;
            let t302 = t301 * t40;
            let t303 = t302 * t89;
            let t304 = f64x8::splat(4.0) * t303;
            let t305 = f64x8::splat(1.0) / t148;
            let t306 = t37 * t305;
            let t307 = t306 * t89;
            let t308 = f64x8::splat(4.0) * t307;
            let t309 = f64x8::splat(1.0) / t38;
            let t310 = t35 * t309;
            let t311 = t42 - t310;
            let t314 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t311));
            let t315 = -t311;
            let t318 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t315));
            let t319 = t314 + t318;
            let t320 = t319 * t60;
            let t321 = t320 * t88;
            let t322 = t41 * t321;
            let t326 = t68 * t68;
            let t327 = f64x8::splat(1.0) / t326;
            let t328 = t63 * t327;
            let t333 = -f64x8::splat(1.176575) * t284 - f64x8::splat(0.516475) * t286 - f64x8::splat(0.2103875) * t290 - f64x8::splat(0.104195) * t294;
            let t334 = f64x8::splat(1.0) / t71;
            let t335 = t333 * t334;
            let t341 = t81 * t81;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t76 * t342;
            let t348 = -f64x8::splat(0.8630833333333333) * t284 - f64x8::splat(0.301925) * t286 - f64x8::splat(0.05501625) * t290 - f64x8::splat(0.082785) * t294;
            let t349 = f64x8::splat(1.0) / t84;
            let t350 = t348 * t349;
            let t353 = f64x8::splat(0.0005323764196666666) * t5 * t273 * t72 + f64x8::splat(1.0) * t328 * t335 - t276 - t300 + f64x8::splat(0.00018311447306006544) * t5 * t273 * t85 + f64x8::splat(0.5848223622634646) * t343 * t350;
            let t354 = t61 * t353;
            let t355 = t41 * t354;
            let t356 = t320 * t86;
            let t357 = f64x8::splat(0.0197516734986138) * t356;
            let t358 = t61 * t2;
            let t360 = t282 * t272 * t85;
            let t361 = t358 * t360;
            let t362 = f64x8::splat(0.00018311447306006544) * t361;
            let t363 = t61 * t76;
            let t365 = t342 * t348 * t349;
            let t366 = t363 * t365;
            let t367 = f64x8::splat(0.5848223622634646) * t366;
            let t368 = t105 * t145;
            let t369 = f64x8::splat(1.0) / t48;
            let t372 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t369 * t311));
            let t373 = f64x8::splat(1.0) / t53;
            let t376 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t373 * t315));
            let t378 = t372 / f64x8::splat(2.0) + t376 / f64x8::splat(2.0);
            let t380 = t97 * t368 * t378;
            let t381 = f64x8::splat(3.0) * t380;
            let t383 = f64x8::splat(1.0) / t139 / t138;
            let t384 = t23 * t198;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t385 * t111;
            let t388 = t121 * t124;
            let t389 = t57 * t130;
            let t390 = t388 * t389;
            let t392 = f64x8::splat(0.002743937159556463) * t386 * t113 * t390;
            let t393 = t110 * t110;
            let t394 = f64x8::splat(1.0) / t393;
            let t395 = t108 * t394;
            let t396 = t395 * t122;
            let t397 = t124 * t385;
            let t400 = f64x8::splat(0.004878720269691391) * t396 * t397 * t389;
            let t401 = t112 * t113;
            let t402 = t120 * t120;
            let t403 = f64x8::splat(1.0) / t402;
            let t404 = t403 * t124;
            let t406 = t401 * t404 * t128;
            let t407 = t389 * t20;
            let t408 = t132 * t6;
            let t410 = (t276 + t300 + t304 - t308 + t322 + t355 + t357 - t362 - t367) * t113;
            let t412 = t105 * t105;
            let t413 = f64x8::splat(1.0) / t412;
            let t414 = t95 * t413;
            let t415 = t414 * t378;
            let t418 = f64x8::splat(3.0) * t115 * t415 - t410 * t117;
            let t419 = t418 * t119;
            let t421 = t407 * t408 * t419;
            let t424 = f64x8::splat(1.0) / t199;
            let t425 = t424 * t57;
            let t429 = f64x8::splat(0.0640252003896508) * t126 * t425 * t130 * t134;
            let t431 = t401 * t388 * t128;
            let t432 = t57 * t116;
            let t433 = t432 * t20;
            let t435 = t433 * t408 * t378;
            let t438 = -t392 + t400 - f64x8::splat(0.027439371595564633) * t406 * t421 - t429 - f64x8::splat(0.054878743191129266) * t431 * t435;
            let t439 = t383 * t438;
            let t445 = f64x8::splat(0.25) * t439 * t120 + f64x8::splat(1.0) * t141 * t418 * t119;
            let t447 = f64x8::splat(1.0) / t144;
            let t449 = t97 * t106 * t445 * t447;
            let t450 = t39 * t177;
            let t452 = f64x8::splat(5.0) * t450 * t207;
            let t453 = t148 * t176;
            let t454 = t453 * t192;
            let t455 = t196 * t206;
            let t456 = v_rho0 * v_rho0;
            let t458 = f64x8::splat(1.0) / t150 / t456;
            let t459 = v_tau0 * t458;
            let t462 = t311 / f64x8::splat(2.0);
            let t463 = t156 * t462;
            let t466 = -t462;
            let t467 = t166 * t466;
            let t470 = t397 / f64x8::splat(3.0);
            let t472 = ((t174).select(-f64x8::splat(5.0) / f64x8::splat(3.0) * t459 * t157 + f64x8::splat(5.0) / f64x8::splat(3.0) * t153 * t463 + f64x8::splat(5.0) / f64x8::splat(3.0) * t163 * t467 + t470, f64x8::splat(0.0)));
            let t473 = t455 * t472;
            let t476 = t23 * t148;
            let t477 = t476 * t177;
            let t478 = t190 * t190;
            let t479 = f64x8::splat(1.0) / t478;
            let t481 = t455 * t183;
            let t483 = f64x8::splat(3.0) / f64x8::splat(2.0) * t477 * t479 * t481;
            let t484 = t178 * t192;
            let t485 = t194 * t194;
            let t486 = f64x8::splat(1.0) / t485;
            let t487 = t486 * t206;
            let t489 = f64x8::splat(5.0) / f64x8::splat(3.0) * t463 + f64x8::splat(5.0) / f64x8::splat(3.0) * t467;
            let t490 = t487 * t489;
            let t493 = t205 * t205;
            let t494 = f64x8::splat(1.0) / t493;
            let t495 = t196 * t494;
            let t496 = t127 * t176;
            let t498 = f64x8::splat(10.0) / f64x8::splat(3.0) * t496 * t203;
            let t499 = t199 * t175;
            let t500 = t203 * t472;
            let t503 = t39 * t176;
            let t504 = t503 * t192;
            let t506 = t202 * t180 * t182;
            let t507 = t504 * t506;
            let t508 = t201 * t196;
            let t509 = t508 * t489;
            let t512 = -f64x8::splat(2.0) * t200 * t509 + f64x8::splat(2.0) * t499 * t500 + t498 - t507;
            let t513 = t495 * t512;
            let t515 = f64x8::splat(3.0) * t454 * t473 - f64x8::splat(3.0) * t484 * t490 - t484 * t513 + t452 - t483;
            let t516 = ((t210).select(f64x8::splat(0.0), t515));
            let t518 = t211 * t516;
            let t520 = t213 * t516;
            let t522 = t215 * t516;
            let t524 = t217 * t516;
            let t526 = t219 * t516;
            let t531 = t227 * t227;
            let t532 = f64x8::splat(1.0) / t531;
            let t533 = ((t210).select(t515, f64x8::splat(0.0)));
            let t537 = ((t209).select(-f64x8::splat(0.64) * t516 - f64x8::splat(0.8704) * t518 - f64x8::splat(4.607056813647) * t520 + f64x8::splat(12.2462410087) * t522 - f64x8::splat(9.57855118103) * t524 + f64x8::splat(3.101306810232) * t526 - f64x8::splat(0.362942158544) * t221 * t516, -f64x8::splat(1.05) * t532 * t533 * t230));
            let t538 = t537 * t269;
            let t539 = t235 * t235;
            let t540 = f64x8::splat(1.0) / t539;
            let t543 = -f64x8::splat(0.007408333333333334) * t284 - f64x8::splat(0.01046175) * t286;
            let t544 = t540 * t543;
            let t546 = t239 * t250;
            let t550 = f64x8::splat(1.0) / t248 / t247;
            let t551 = t240 * t550;
            let t552 = t551 * t179;
            let t553 = t241 * t187;
            let t557 = -f64x8::splat(1.0) * t544 * t546 - f64x8::splat(0.014225094736250906) * t552 * t553 * t397;
            let t558 = f64x8::splat(1.0) / t252;
            let t561 = f64x8::splat(0.0285764) * t544 + f64x8::splat(0.0285764) * t557 * t558;
            let t562 = t561 * t259;
            let t563 = t562 * t267;
            let t564 = t255 * t58;
            let t565 = t320 * t267;
            let t568 = t261 * t301;
            let t569 = t568 * t265;
            let t570 = t263 * t148;
            let t571 = f64x8::splat(1.0) / t570;
            let t572 = t262 * t571;
            let t574 = -f64x8::splat(12.0) * t569 + f64x8::splat(12.0) * t572;
            let t576 = t563 - f64x8::splat(2.363) * t564 * t565 + t260 * t574 - t276 - t300 - t304 + t308 - t322 - t355 - t357 + t362 + t367 - t381 - t449;
            let t577 = t232 * t576;
            let t578 = t276 + t300 + t304 - t308 + t322 + t355 + t357 - t362 - t367 + t381 + t449 + t538 + t577;
            let tvrho0 = t8 * t578 + t147 + t270 - t34 + t90 + t92;
            acc_vrho_0 = tvrho0;
            let t580 = -t42 - t310;
            let t583 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t580));
            let t584 = -t580;
            let t587 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t584));
            let t588 = t583 + t587;
            let t589 = t588 * t60;
            let t590 = t589 * t88;
            let t591 = t41 * t590;
            let t592 = t589 * t86;
            let t593 = f64x8::splat(0.0197516734986138) * t592;
            let t596 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t369 * t580));
            let t599 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t373 * t584));
            let t601 = t596 / f64x8::splat(2.0) + t599 / f64x8::splat(2.0);
            let t603 = t97 * t368 * t601;
            let t604 = f64x8::splat(3.0) * t603;
            let t606 = (t276 + t300 - t304 - t308 + t591 + t355 + t593 - t362 - t367) * t113;
            let t608 = t414 * t601;
            let t611 = f64x8::splat(3.0) * t115 * t608 - t606 * t117;
            let t612 = t611 * t119;
            let t614 = t407 * t408 * t612;
            let t618 = t433 * t408 * t601;
            let t621 = -t392 + t400 - f64x8::splat(0.027439371595564633) * t406 * t614 - t429 - f64x8::splat(0.054878743191129266) * t431 * t618;
            let t622 = t383 * t621;
            let t625 = t141 * t611;
            let t628 = f64x8::splat(0.25) * t622 * t120 + f64x8::splat(1.0) * t625 * t119;
            let t631 = t97 * t106 * t628 * t447;
            let t632 = t580 / f64x8::splat(2.0);
            let t633 = t156 * t632;
            let t636 = v_rho1 * v_rho1;
            let t638 = f64x8::splat(1.0) / t160 / t636;
            let t639 = v_tau1 * t638;
            let t642 = -t632;
            let t643 = t166 * t642;
            let t647 = ((t174).select(f64x8::splat(5.0) / f64x8::splat(3.0) * t153 * t633 - f64x8::splat(5.0) / f64x8::splat(3.0) * t639 * t167 + f64x8::splat(5.0) / f64x8::splat(3.0) * t163 * t643 + t470, f64x8::splat(0.0)));
            let t648 = t455 * t647;
            let t652 = f64x8::splat(5.0) / f64x8::splat(3.0) * t633 + f64x8::splat(5.0) / f64x8::splat(3.0) * t643;
            let t653 = t487 * t652;
            let t656 = t203 * t647;
            let t659 = t508 * t652;
            let t662 = -f64x8::splat(2.0) * t200 * t659 + f64x8::splat(2.0) * t499 * t656 + t498 - t507;
            let t663 = t495 * t662;
            let t665 = f64x8::splat(3.0) * t454 * t648 - f64x8::splat(3.0) * t484 * t653 - t484 * t663 + t452 - t483;
            let t666 = ((t210).select(f64x8::splat(0.0), t665));
            let t668 = t211 * t666;
            let t670 = t213 * t666;
            let t672 = t215 * t666;
            let t674 = t217 * t666;
            let t676 = t219 * t666;
            let t681 = ((t210).select(t665, f64x8::splat(0.0)));
            let t685 = ((t209).select(-f64x8::splat(0.64) * t666 - f64x8::splat(0.8704) * t668 - f64x8::splat(4.607056813647) * t670 + f64x8::splat(12.2462410087) * t672 - f64x8::splat(9.57855118103) * t674 + f64x8::splat(3.101306810232) * t676 - f64x8::splat(0.362942158544) * t221 * t666, -f64x8::splat(1.05) * t532 * t681 * t230));
            let t686 = t685 * t269;
            let t687 = t589 * t267;
            let t691 = f64x8::splat(12.0) * t569 + f64x8::splat(12.0) * t572;
            let t693 = t563 - f64x8::splat(2.363) * t564 * t687 + t260 * t691 - t276 - t300 + t304 + t308 - t591 - t355 - t593 + t362 + t367 - t604 - t631;
            let t694 = t232 * t693;
            let t695 = t276 + t300 - t304 - t308 + t591 + t355 + t593 - t362 - t367 + t604 + t631 + t686 + t694;
            let tvrho1 = t8 * t695 + t147 + t270 - t34 + t90 + t92;
            acc_vrho_1 = tvrho1;
            let t697 = t104 * t383;
            let t698 = t112 * t128;
            let t699 = t697 * t698;
            let t700 = t57 * t20;
            let t701 = t408 * t447;
            let t702 = t700 * t701;
            let t703 = t699 * t702;
            let t704 = f64x8::splat(0.0006950474021161377) * t703;
            let t706 = ((t174).select(-t170 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t707 = t455 * t706;
            let t710 = t9 * t263;
            let t711 = t176 * t176;
            let t712 = t710 * t711;
            let t714 = f64x8::splat(1.0) / t478 / t189;
            let t715 = t712 * t714;
            let t717 = f64x8::splat(1.0) / t485 / t193;
            let t718 = t717 * t494;
            let t719 = t718 * t706;
            let t722 = f64x8::splat(3.0) * t454 * t707 - f64x8::splat(2.0) * t715 * t719;
            let t723 = ((t210).select(f64x8::splat(0.0), t722));
            let t725 = t211 * t723;
            let t727 = t213 * t723;
            let t729 = t215 * t723;
            let t731 = t217 * t723;
            let t733 = t219 * t723;
            let t738 = ((t210).select(t722, f64x8::splat(0.0)));
            let t742 = ((t209).select(-f64x8::splat(0.64) * t723 - f64x8::splat(0.8704) * t725 - f64x8::splat(4.607056813647) * t727 + f64x8::splat(12.2462410087) * t729 - f64x8::splat(9.57855118103) * t731 + f64x8::splat(3.101306810232) * t733 - f64x8::splat(0.362942158544) * t221 * t723, -f64x8::splat(1.05) * t532 * t738 * t230));
            let t743 = t742 * t269;
            let t744 = t551 * t242;
            let t745 = t187 * t170;
            let t746 = t558 * t259;
            let t747 = t746 * t267;
            let t749 = t744 * t745 * t747;
            let t751 = f64x8::splat(0.00015243824895787514) * t749 - t704;
            let t752 = t232 * t751;
            let tvsigma0 = t8 * (t704 + t743 + t752);
            acc_vsigma_0 = tvsigma0;
            let t754 = f64x8::splat(0.0013900948042322753) * t703;
            let t756 = ((t174).select(-t170 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t757 = t455 * t756;
            let t760 = t718 * t756;
            let t763 = f64x8::splat(3.0) * t454 * t757 - f64x8::splat(2.0) * t715 * t760;
            let t764 = ((t210).select(f64x8::splat(0.0), t763));
            let t766 = t211 * t764;
            let t768 = t213 * t764;
            let t770 = t215 * t764;
            let t772 = t217 * t764;
            let t774 = t219 * t764;
            let t779 = ((t210).select(t763, f64x8::splat(0.0)));
            let t783 = ((t209).select(-f64x8::splat(0.64) * t764 - f64x8::splat(0.8704) * t766 - f64x8::splat(4.607056813647) * t768 + f64x8::splat(12.2462410087) * t770 - f64x8::splat(9.57855118103) * t772 + f64x8::splat(3.101306810232) * t774 - f64x8::splat(0.362942158544) * t221 * t764, -f64x8::splat(1.05) * t532 * t779 * t230));
            let t784 = t783 * t269;
            let t786 = f64x8::splat(0.0003048764979157503) * t749 - t754;
            let t787 = t232 * t786;
            let tvsigma1 = t8 * (t754 + t784 + t787);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t790 = ((t174).select(t152 * t157, f64x8::splat(0.0)));
            let t791 = t455 * t790;
            let t794 = t718 * t790;
            let t797 = f64x8::splat(3.0) * t454 * t791 - f64x8::splat(2.0) * t715 * t794;
            let t798 = ((t210).select(f64x8::splat(0.0), t797));
            let t800 = t211 * t798;
            let t802 = t213 * t798;
            let t804 = t215 * t798;
            let t806 = t217 * t798;
            let t808 = t219 * t798;
            let t813 = ((t210).select(t797, f64x8::splat(0.0)));
            let t817 = ((t209).select(-f64x8::splat(0.64) * t798 - f64x8::splat(0.8704) * t800 - f64x8::splat(4.607056813647) * t802 + f64x8::splat(12.2462410087) * t804 - f64x8::splat(9.57855118103) * t806 + f64x8::splat(3.101306810232) * t808 - f64x8::splat(0.362942158544) * t221 * t798, -f64x8::splat(1.05) * t532 * t813 * t230));
            let t818 = t8 * t817;
            let tvtau0 = t818 * t269;
            acc_vtau_0 = tvtau0;
            let t820 = ((t174).select(t162 * t167, f64x8::splat(0.0)));
            let t821 = t455 * t820;
            let t824 = t718 * t820;
            let t827 = f64x8::splat(3.0) * t454 * t821 - f64x8::splat(2.0) * t715 * t824;
            let t828 = ((t210).select(f64x8::splat(0.0), t827));
            let t830 = t211 * t828;
            let t832 = t213 * t828;
            let t834 = t215 * t828;
            let t836 = t217 * t828;
            let t838 = t219 * t828;
            let t843 = ((t210).select(t827, f64x8::splat(0.0)));
            let t847 = ((t209).select(-f64x8::splat(0.64) * t828 - f64x8::splat(0.8704) * t830 - f64x8::splat(4.607056813647) * t832 + f64x8::splat(12.2462410087) * t834 - f64x8::splat(9.57855118103) * t836 + f64x8::splat(3.101306810232) * t838 - f64x8::splat(0.362942158544) * t221 * t828, -f64x8::splat(1.05) * t532 * t843 * t230));
            let t848 = t8 * t847;
            let tvtau1 = t848 * t269;
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
