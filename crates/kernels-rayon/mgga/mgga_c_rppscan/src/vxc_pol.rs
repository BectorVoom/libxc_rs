//! MGGA_C_RPPSCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rppscan.c`
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
pub fn mgga_c_rppscan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_eta = f64x8::splat(param_eta);
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
            let t26 = t22 * t6 / t23;
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
            let t128 = f64x8::splat(1.0) / t9 / t38;
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
            let t148 = (simd::cbrt(v_rho0));
            let t149 = t148 * t148;
            let t151 = f64x8::splat(1.0) / t149 / v_rho0;
            let t152 = v_tau0 * t151;
            let t153 = t44 / f64x8::splat(2.0);
            let t154 = (simd::cbrt(t153));
            let t155 = t154 * t154;
            let t156 = t155 * t153;
            let t158 = (simd::cbrt(v_rho1));
            let t159 = t158 * t158;
            let t161 = f64x8::splat(1.0) / t159 / v_rho1;
            let t162 = v_tau1 * t161;
            let t163 = t51 / f64x8::splat(2.0);
            let t164 = (simd::cbrt(t163));
            let t165 = t164 * t164;
            let t166 = t165 * t163;
            let t169 = f64x8::splat(1.0) / t23 / t38;
            let t172 = t152 * t156 + t162 * t166 - t124 * t169 / f64x8::splat(8.0);
            let t173 = f64x8::splat(M_CBRT6);
            let t174 = t173 * t173;
            let t175 = (simd::cbrt(t95));
            let t176 = t175 * t175;
            let t177 = t174 * t176;
            let t181 = param_eta * t124;
            let t184 = f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * (t156 + t166) + t181 * t169 / f64x8::splat(8.0);
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t172 * t185;
            let t187 = (t186).simd_le(f64x8::splat(2.5));
            let t188 = (f64x8::splat(2.5)).simd_lt(t186);
            let t189 = ((t188).select(f64x8::splat(2.5), t186));
            let t191 = t189 * t189;
            let t193 = t191 * t189;
            let t195 = t191 * t191;
            let t197 = t195 * t189;
            let t199 = t195 * t191;
            let t204 = ((t188).select(t186, f64x8::splat(2.5)));
            let t205 = f64x8::splat(1.0) - t204;
            let t208 = (simd::exp(f64x8::splat(1.5) / t205));
            let t210 = ((t187).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t189 - f64x8::splat(0.4352) * t191 - f64x8::splat(1.535685604549) * t193 + f64x8::splat(3.061560252175) * t195 - f64x8::splat(1.915710236206) * t197 + f64x8::splat(0.516884468372) * t199 - f64x8::splat(0.051848879792) * t195 * t193, -f64x8::splat(0.7) * t208));
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + f64x8::splat(0.03138525) * t12;
            let t214 = f64x8::splat(1.0) / t213;
            let t217 = (simd::exp(f64x8::splat(1.0) * t214));
            let t218 = t217 - f64x8::splat(1.0);
            let t219 = f64x8::splat(1.0) / t176;
            let t220 = t173 * t219;
            let t221 = t57 * t57;
            let t222 = t221 * t124;
            let t226 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t220 * t222 * t169;
            let t227 = ((t226).sqrt().sqrt());
            let t229 = f64x8::splat(1.0) - f64x8::splat(1.0) / t227;
            let t231 = t218 * t229 + f64x8::splat(1.0);
            let t232 = (simd::ln(t231));
            let t234 = -f64x8::splat(0.0285764) * t214 + f64x8::splat(0.0285764) * t232;
            let t238 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t239 = t234 * t238;
            let t240 = t37 * t37;
            let t241 = t240 * t37;
            let t242 = t39 * t39;
            let t243 = t242 * t39;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = -t241 * t244 + f64x8::splat(1.0);
            let t248 = t239 * t246 - t147 + t34 - t90 - t92;
            let t249 = t210 * t248;
            let tzk0 = -t34 + t90 + t92 + t147 + t249;
            acc_zk = tzk0;
            let t251 = f64x8::splat(1.0) / t9 / t8;
            let t252 = t7 * t251;
            let t254 = t5 * t252 * t32;
            let t255 = f64x8::splat(0.0011073470983333333) * t254;
            let t256 = t28 * t28;
            let t257 = f64x8::splat(1.0) / t256;
            let t258 = t14 * t257;
            let t260 = f64x8::splat(1.0) / t15 * t2;
            let t261 = t4 * t7;
            let t262 = t261 * t251;
            let t263 = t260 * t262;
            let t265 = t5 * t252;
            let t267 = ((t12).sqrt());
            let t268 = t267 * t2;
            let t269 = t268 * t262;
            let t274 = t22 * t6 / t23 / t8;
            let t276 = -f64x8::splat(0.632975) * t263 - f64x8::splat(0.29896666666666666) * t265 - f64x8::splat(0.1023875) * t269 - f64x8::splat(0.08215666666666667) * t274;
            let t277 = f64x8::splat(1.0) / t31;
            let t278 = t276 * t277;
            let t279 = t258 * t278;
            let t280 = f64x8::splat(1.0) * t279;
            let t281 = t36 * t35;
            let t282 = t281 * t40;
            let t283 = t282 * t89;
            let t284 = f64x8::splat(4.0) * t283;
            let t285 = t39 * t8;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t37 * t286;
            let t288 = t287 * t89;
            let t289 = f64x8::splat(4.0) * t288;
            let t290 = f64x8::splat(1.0) / t38;
            let t291 = t35 * t290;
            let t292 = t42 - t291;
            let t295 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t292));
            let t296 = -t292;
            let t299 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t296));
            let t300 = t295 + t299;
            let t301 = t300 * t60;
            let t302 = t301 * t88;
            let t303 = t41 * t302;
            let t307 = t68 * t68;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t63 * t308;
            let t314 = -f64x8::splat(1.176575) * t263 - f64x8::splat(0.516475) * t265 - f64x8::splat(0.2103875) * t269 - f64x8::splat(0.104195) * t274;
            let t315 = f64x8::splat(1.0) / t71;
            let t316 = t314 * t315;
            let t322 = t81 * t81;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t76 * t323;
            let t329 = -f64x8::splat(0.8630833333333333) * t263 - f64x8::splat(0.301925) * t265 - f64x8::splat(0.05501625) * t269 - f64x8::splat(0.082785) * t274;
            let t330 = f64x8::splat(1.0) / t84;
            let t331 = t329 * t330;
            let t334 = f64x8::splat(0.0005323764196666666) * t5 * t252 * t72 + f64x8::splat(1.0) * t309 * t316 - t255 - t280 + f64x8::splat(0.00018311447306006544) * t5 * t252 * t85 + f64x8::splat(0.5848223622634646) * t324 * t331;
            let t335 = t61 * t334;
            let t336 = t41 * t335;
            let t337 = t301 * t86;
            let t338 = f64x8::splat(0.0197516734986138) * t337;
            let t339 = t61 * t2;
            let t341 = t261 * t251 * t85;
            let t342 = t339 * t341;
            let t343 = f64x8::splat(0.00018311447306006544) * t342;
            let t344 = t61 * t76;
            let t346 = t323 * t329 * t330;
            let t347 = t344 * t346;
            let t348 = f64x8::splat(0.5848223622634646) * t347;
            let t349 = t105 * t145;
            let t350 = f64x8::splat(1.0) / t48;
            let t353 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t292));
            let t354 = f64x8::splat(1.0) / t53;
            let t357 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t354 * t296));
            let t359 = t353 / f64x8::splat(2.0) + t357 / f64x8::splat(2.0);
            let t361 = t97 * t349 * t359;
            let t362 = f64x8::splat(3.0) * t361;
            let t364 = f64x8::splat(1.0) / t139 / t138;
            let t365 = t38 * t8;
            let t367 = f64x8::splat(1.0) / t23 / t365;
            let t368 = t367 * t111;
            let t370 = t121 * t124;
            let t371 = t57 * t130;
            let t372 = t370 * t371;
            let t374 = f64x8::splat(0.002743937159556463) * t368 * t113 * t372;
            let t375 = t110 * t110;
            let t376 = f64x8::splat(1.0) / t375;
            let t377 = t108 * t376;
            let t378 = t377 * t122;
            let t379 = t124 * t367;
            let t382 = f64x8::splat(0.004878720269691391) * t378 * t379 * t371;
            let t383 = t112 * t113;
            let t384 = t120 * t120;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t385 * t124;
            let t388 = t383 * t386 * t128;
            let t389 = t371 * t20;
            let t390 = t132 * t6;
            let t392 = (t255 + t280 + t284 - t289 + t303 + t336 + t338 - t343 - t348) * t113;
            let t394 = t105 * t105;
            let t395 = f64x8::splat(1.0) / t394;
            let t396 = t95 * t395;
            let t397 = t396 * t359;
            let t400 = f64x8::splat(3.0) * t115 * t397 - t392 * t117;
            let t401 = t400 * t119;
            let t403 = t389 * t390 * t401;
            let t407 = f64x8::splat(1.0) / t9 / t365;
            let t408 = t407 * t57;
            let t412 = f64x8::splat(0.0640252003896508) * t126 * t408 * t130 * t134;
            let t414 = t383 * t370 * t128;
            let t415 = t57 * t116;
            let t416 = t415 * t20;
            let t418 = t416 * t390 * t359;
            let t421 = -t374 + t382 - f64x8::splat(0.027439371595564633) * t388 * t403 - t412 - f64x8::splat(0.054878743191129266) * t414 * t418;
            let t422 = t364 * t421;
            let t428 = f64x8::splat(0.25) * t422 * t120 + f64x8::splat(1.0) * t141 * t400 * t119;
            let t430 = f64x8::splat(1.0) / t144;
            let t432 = t97 * t106 * t428 * t430;
            let t433 = v_rho0 * v_rho0;
            let t435 = f64x8::splat(1.0) / t149 / t433;
            let t436 = v_tau0 * t435;
            let t439 = t292 / f64x8::splat(2.0);
            let t440 = t155 * t439;
            let t443 = -t439;
            let t444 = t165 * t443;
            let t447 = t379 / f64x8::splat(3.0);
            let t448 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t436 * t156 + f64x8::splat(5.0) / f64x8::splat(3.0) * t152 * t440 + f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t444 + t447;
            let t450 = t184 * t184;
            let t451 = f64x8::splat(1.0) / t450;
            let t452 = t172 * t451;
            let t458 = t181 * t367 / f64x8::splat(3.0);
            let t459 = f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * (f64x8::splat(5.0) / f64x8::splat(3.0) * t440 + f64x8::splat(5.0) / f64x8::splat(3.0) * t444) - t458;
            let t461 = t448 * t185 - t452 * t459;
            let t462 = ((t188).select(f64x8::splat(0.0), t461));
            let t464 = t189 * t462;
            let t466 = t191 * t462;
            let t468 = t193 * t462;
            let t470 = t195 * t462;
            let t472 = t197 * t462;
            let t477 = t205 * t205;
            let t478 = f64x8::splat(1.0) / t477;
            let t479 = ((t188).select(t461, f64x8::splat(0.0)));
            let t483 = ((t187).select(-f64x8::splat(0.64) * t462 - f64x8::splat(0.8704) * t464 - f64x8::splat(4.607056813647) * t466 + f64x8::splat(12.2462410087) * t468 - f64x8::splat(9.57855118103) * t470 + f64x8::splat(3.101306810232) * t472 - f64x8::splat(0.362942158544) * t199 * t462, -f64x8::splat(1.05) * t478 * t479 * t208));
            let t484 = t483 * t248;
            let t485 = t213 * t213;
            let t486 = f64x8::splat(1.0) / t485;
            let t489 = -f64x8::splat(0.007408333333333334) * t263 - f64x8::splat(0.01046175) * t265;
            let t490 = t486 * t489;
            let t492 = t217 * t229;
            let t496 = f64x8::splat(1.0) / t227 / t226;
            let t497 = t218 * t496;
            let t498 = t497 * t173;
            let t499 = t219 * t221;
            let t503 = -f64x8::splat(1.0) * t490 * t492 - f64x8::splat(0.014225094736250906) * t498 * t499 * t379;
            let t504 = f64x8::splat(1.0) / t231;
            let t507 = f64x8::splat(0.0285764) * t490 + f64x8::splat(0.0285764) * t503 * t504;
            let t508 = t507 * t238;
            let t509 = t508 * t246;
            let t510 = t234 * t58;
            let t511 = t301 * t246;
            let t514 = t240 * t281;
            let t515 = t514 * t244;
            let t516 = t242 * t285;
            let t517 = f64x8::splat(1.0) / t516;
            let t518 = t241 * t517;
            let t520 = -f64x8::splat(12.0) * t515 + f64x8::splat(12.0) * t518;
            let t522 = t509 - f64x8::splat(2.363) * t510 * t511 + t239 * t520 - t255 - t280 - t284 + t289 - t303 - t336 - t338 + t343 + t348 - t362 - t432;
            let t523 = t210 * t522;
            let t524 = t255 + t280 + t284 - t289 + t303 + t336 + t338 - t343 - t348 + t362 + t432 + t484 + t523;
            let tvrho0 = t8 * t524 + t147 + t249 - t34 + t90 + t92;
            acc_vrho_0 = tvrho0;
            let t526 = -t42 - t291;
            let t529 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t526));
            let t530 = -t526;
            let t533 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t530));
            let t534 = t529 + t533;
            let t535 = t534 * t60;
            let t536 = t535 * t88;
            let t537 = t41 * t536;
            let t538 = t535 * t86;
            let t539 = f64x8::splat(0.0197516734986138) * t538;
            let t542 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t526));
            let t545 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t354 * t530));
            let t547 = t542 / f64x8::splat(2.0) + t545 / f64x8::splat(2.0);
            let t549 = t97 * t349 * t547;
            let t550 = f64x8::splat(3.0) * t549;
            let t552 = (t255 + t280 - t284 - t289 + t537 + t336 + t539 - t343 - t348) * t113;
            let t554 = t396 * t547;
            let t557 = f64x8::splat(3.0) * t115 * t554 - t552 * t117;
            let t558 = t557 * t119;
            let t560 = t389 * t390 * t558;
            let t564 = t416 * t390 * t547;
            let t567 = -t374 + t382 - f64x8::splat(0.027439371595564633) * t388 * t560 - t412 - f64x8::splat(0.054878743191129266) * t414 * t564;
            let t568 = t364 * t567;
            let t571 = t141 * t557;
            let t574 = f64x8::splat(0.25) * t568 * t120 + f64x8::splat(1.0) * t571 * t119;
            let t577 = t97 * t106 * t574 * t430;
            let t578 = t526 / f64x8::splat(2.0);
            let t579 = t155 * t578;
            let t582 = v_rho1 * v_rho1;
            let t584 = f64x8::splat(1.0) / t159 / t582;
            let t585 = v_tau1 * t584;
            let t588 = -t578;
            let t589 = t165 * t588;
            let t592 = f64x8::splat(5.0) / f64x8::splat(3.0) * t152 * t579 - f64x8::splat(5.0) / f64x8::splat(3.0) * t585 * t166 + f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t589 + t447;
            let t598 = f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * (f64x8::splat(5.0) / f64x8::splat(3.0) * t579 + f64x8::splat(5.0) / f64x8::splat(3.0) * t589) - t458;
            let t600 = t592 * t185 - t452 * t598;
            let t601 = ((t188).select(f64x8::splat(0.0), t600));
            let t603 = t189 * t601;
            let t605 = t191 * t601;
            let t607 = t193 * t601;
            let t609 = t195 * t601;
            let t611 = t197 * t601;
            let t616 = ((t188).select(t600, f64x8::splat(0.0)));
            let t620 = ((t187).select(-f64x8::splat(0.64) * t601 - f64x8::splat(0.8704) * t603 - f64x8::splat(4.607056813647) * t605 + f64x8::splat(12.2462410087) * t607 - f64x8::splat(9.57855118103) * t609 + f64x8::splat(3.101306810232) * t611 - f64x8::splat(0.362942158544) * t199 * t601, -f64x8::splat(1.05) * t478 * t616 * t208));
            let t621 = t620 * t248;
            let t622 = t535 * t246;
            let t626 = f64x8::splat(12.0) * t515 + f64x8::splat(12.0) * t518;
            let t628 = t509 - f64x8::splat(2.363) * t510 * t622 + t239 * t626 - t255 - t280 + t284 + t289 - t537 - t336 - t539 + t343 + t348 - t550 - t577;
            let t629 = t210 * t628;
            let t630 = t255 + t280 - t284 - t289 + t537 + t336 + t539 - t343 - t348 + t550 + t577 + t621 + t629;
            let tvrho1 = t8 * t630 + t147 + t249 - t34 + t90 + t92;
            acc_vrho_1 = tvrho1;
            let t632 = t104 * t364;
            let t633 = t112 * t128;
            let t634 = t632 * t633;
            let t635 = t57 * t20;
            let t636 = t390 * t430;
            let t637 = t635 * t636;
            let t638 = t634 * t637;
            let t639 = f64x8::splat(0.0006950474021161377) * t638;
            let t641 = param_eta * t169;
            let t643 = -t169 * t185 - t452 * t641;
            let t644 = t643 / f64x8::splat(8.0);
            let t645 = ((t188).select(f64x8::splat(0.0), t644));
            let t647 = t189 * t645;
            let t649 = t191 * t645;
            let t651 = t193 * t645;
            let t653 = t195 * t645;
            let t655 = t197 * t645;
            let t660 = ((t188).select(t644, f64x8::splat(0.0)));
            let t664 = ((t187).select(-f64x8::splat(0.64) * t645 - f64x8::splat(0.8704) * t647 - f64x8::splat(4.607056813647) * t649 + f64x8::splat(12.2462410087) * t651 - f64x8::splat(9.57855118103) * t653 + f64x8::splat(3.101306810232) * t655 - f64x8::splat(0.362942158544) * t199 * t645, -f64x8::splat(1.05) * t478 * t660 * t208));
            let t665 = t664 * t248;
            let t666 = t497 * t220;
            let t667 = t221 * t169;
            let t668 = t504 * t238;
            let t669 = t668 * t246;
            let t671 = t666 * t667 * t669;
            let t673 = f64x8::splat(0.00015243824895787514) * t671 - t639;
            let t674 = t210 * t673;
            let tvsigma0 = t8 * (t639 + t665 + t674);
            acc_vsigma_0 = tvsigma0;
            let t676 = f64x8::splat(0.0013900948042322753) * t638;
            let t677 = t643 / f64x8::splat(4.0);
            let t678 = ((t188).select(f64x8::splat(0.0), t677));
            let t680 = t189 * t678;
            let t682 = t191 * t678;
            let t684 = t193 * t678;
            let t686 = t195 * t678;
            let t688 = t197 * t678;
            let t693 = ((t188).select(t677, f64x8::splat(0.0)));
            let t697 = ((t187).select(-f64x8::splat(0.64) * t678 - f64x8::splat(0.8704) * t680 - f64x8::splat(4.607056813647) * t682 + f64x8::splat(12.2462410087) * t684 - f64x8::splat(9.57855118103) * t686 + f64x8::splat(3.101306810232) * t688 - f64x8::splat(0.362942158544) * t199 * t678, -f64x8::splat(1.05) * t478 * t693 * t208));
            let t698 = t697 * t248;
            let t700 = f64x8::splat(0.0003048764979157503) * t671 - t676;
            let t701 = t210 * t700;
            let tvsigma1 = t8 * (t676 + t698 + t701);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t703 = t151 * t156;
            let t704 = t703 * t185;
            let t705 = ((t188).select(f64x8::splat(0.0), t704));
            let t707 = t189 * t705;
            let t709 = t191 * t705;
            let t711 = t193 * t705;
            let t713 = t195 * t705;
            let t715 = t197 * t705;
            let t720 = ((t188).select(t704, f64x8::splat(0.0)));
            let t724 = ((t187).select(-f64x8::splat(0.64) * t705 - f64x8::splat(0.8704) * t707 - f64x8::splat(4.607056813647) * t709 + f64x8::splat(12.2462410087) * t711 - f64x8::splat(9.57855118103) * t713 + f64x8::splat(3.101306810232) * t715 - f64x8::splat(0.362942158544) * t199 * t705, -f64x8::splat(1.05) * t478 * t720 * t208));
            let t725 = t8 * t724;
            let tvtau0 = t725 * t248;
            acc_vtau_0 = tvtau0;
            let t726 = t161 * t166;
            let t727 = t726 * t185;
            let t728 = ((t188).select(f64x8::splat(0.0), t727));
            let t730 = t189 * t728;
            let t732 = t191 * t728;
            let t734 = t193 * t728;
            let t736 = t195 * t728;
            let t738 = t197 * t728;
            let t743 = ((t188).select(t727, f64x8::splat(0.0)));
            let t747 = ((t187).select(-f64x8::splat(0.64) * t728 - f64x8::splat(0.8704) * t730 - f64x8::splat(4.607056813647) * t732 + f64x8::splat(12.2462410087) * t734 - f64x8::splat(9.57855118103) * t736 + f64x8::splat(3.101306810232) * t738 - f64x8::splat(0.362942158544) * t199 * t728, -f64x8::splat(1.05) * t478 * t743 * t208));
            let t748 = t8 * t747;
            let tvtau1 = t748 * t248;
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
