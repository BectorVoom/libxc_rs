//! MGGA_C_CCALDA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
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
pub fn mgga_c_ccalda_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c = f64x8::splat(param_c);
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
            let t2 = f64x8::splat(1.0) + param_c;
            let t3 = (simd::cbrt(v_rho0));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / v_rho0;
            let t7 = v_tau0 * t6;
            let t8 = v_rho0 - v_rho1;
            let t9 = v_rho0 + v_rho1;
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t8 * t10;
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = t12 / f64x8::splat(2.0);
            let t14 = (simd::cbrt(t13));
            let t15 = t14 * t14;
            let t16 = t15 * t13;
            let t17 = t7 * t16;
            let t18 = (simd::cbrt(v_rho1));
            let t19 = t18 * t18;
            let t21 = f64x8::splat(1.0) / t19 / v_rho1;
            let t22 = v_tau1 * t21;
            let t23 = f64x8::splat(1.0) - t11;
            let t24 = t23 / f64x8::splat(2.0);
            let t25 = (simd::cbrt(t24));
            let t26 = t25 * t25;
            let t27 = t26 * t24;
            let t28 = t22 * t27;
            let t30 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t31 = t9 * t9;
            let t32 = (simd::cbrt(t9));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / t31;
            let t38 = t17 + t28 - t30 * t35 / f64x8::splat(8.0);
            let t39 = t2 * t38;
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t42 = (simd::cbrt(t41));
            let t43 = t42 * t42;
            let t44 = f64x8::splat(1.0) / t43;
            let t45 = t40 * t44;
            let t46 = t39 * t45;
            let t47 = f64x8::splat(M_CBRT2);
            let t48 = t47 * t47;
            let t50 = t45 * t48;
            let t53 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c * t38 * t50;
            let t54 = f64x8::splat(1.0) / t53;
            let t55 = t48 * t54;
            let t56 = t31 * t31;
            let t58 = f64x8::splat(1.0) / t33 / t56;
            let t59 = t30 * t58;
            let t60 = t17 + t28;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t8 * t8;
            let t63 = t61 * t62;
            let t66 = f64x8::splat(1.0) - t59 * t63 / f64x8::splat(8.0);
            let t67 = f64x8::splat(M_CBRT3);
            let t68 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t69 = (simd::cbrt(t68));
            let t70 = t67 * t69;
            let t71 = f64x8::splat(M_CBRT4);
            let t72 = t71 * t71;
            let t75 = t70 * t72 / t32;
            let t77 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t75;
            let t78 = ((t75).sqrt());
            let t81 = ((t75) * (t75).sqrt());
            let t83 = t67 * t67;
            let t84 = t69 * t69;
            let t85 = t83 * t84;
            let t88 = t85 * t71 / t33;
            let t90 = f64x8::splat(3.79785) * t78 + f64x8::splat(0.8969) * t75 + f64x8::splat(0.204775) * t81 + f64x8::splat(0.123235) * t88;
            let t93 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t90;
            let t94 = (simd::ln(t93));
            let t96 = f64x8::splat(0.0621814) * t77 * t94;
            let t97 = t62 * t62;
            let t98 = f64x8::splat(1.0) / t56;
            let t99 = t97 * t98;
            let t100 = (t12).simd_le(zeta_threshold);
            let t101 = (simd::cbrt(zeta_threshold));
            let t102 = t101 * zeta_threshold;
            let t103 = (simd::cbrt(t12));
            let t105 = ((t100).select(t102, t103 * t12));
            let t106 = (t23).simd_le(zeta_threshold);
            let t107 = (simd::cbrt(t23));
            let t109 = ((t106).select(t102, t107 * t23));
            let t110 = t105 + t109 - f64x8::splat(2.0);
            let t113 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t47 - f64x8::splat(2.0));
            let t114 = t110 * t113;
            let t116 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t75;
            let t121 = f64x8::splat(7.05945) * t78 + f64x8::splat(1.549425) * t75 + f64x8::splat(0.420775) * t81 + f64x8::splat(0.1562925) * t88;
            let t124 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t121;
            let t125 = (simd::ln(t124));
            let t129 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t75;
            let t134 = f64x8::splat(5.1785) * t78 + f64x8::splat(0.905775) * t75 + f64x8::splat(0.1100325) * t81 + f64x8::splat(0.1241775) * t88;
            let t137 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t134;
            let t138 = (simd::ln(t137));
            let t139 = t129 * t138;
            let t141 = -f64x8::splat(0.0310907) * t116 * t125 + t96 - f64x8::splat(0.0197516734986138) * t139;
            let t142 = t114 * t141;
            let t146 = -t96 + t99 * t142 + f64x8::splat(0.0197516734986138) * t114 * t139;
            let t147 = t66 * t146;
            let t148 = t55 * t147;
            let t150 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t148;
            let t151 = t39 * t40;
            let t152 = t44 * t48;
            let t153 = t152 * t54;
            let t154 = t151 * t153;
            let t156 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t154;
            let t157 = t156 * t146;
            let tzk0 = t150 + t157;
            acc_zk = tzk0;
            let t158 = v_rho0 * v_rho0;
            let t160 = f64x8::splat(1.0) / t4 / t158;
            let t161 = v_tau0 * t160;
            let t162 = t161 * t16;
            let t164 = f64x8::splat(1.0) / t31;
            let t165 = t8 * t164;
            let t166 = t10 - t165;
            let t167 = t166 / f64x8::splat(2.0);
            let t168 = t15 * t167;
            let t169 = t7 * t168;
            let t171 = -t167;
            let t172 = t26 * t171;
            let t173 = t22 * t172;
            let t175 = t31 * t9;
            let t177 = f64x8::splat(1.0) / t33 / t175;
            let t179 = t30 * t177 / f64x8::splat(3.0);
            let t180 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t162 + f64x8::splat(5.0) / f64x8::splat(3.0) * t169 + f64x8::splat(5.0) / f64x8::splat(3.0) * t173 + t179;
            let t181 = t2 * t180;
            let t182 = t181 * t45;
            let t183 = t182 * t148;
            let t184 = f64x8::splat(5.0) / f64x8::splat(9.0) * t183;
            let t185 = t40 * t40;
            let t187 = f64x8::splat(1.0) / t42 / t41;
            let t188 = t185 * t187;
            let t189 = t188 * t47;
            let t190 = t39 * t189;
            let t191 = t53 * t53;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t192 * t66;
            let t194 = t146 * param_c;
            let t195 = t194 * t180;
            let t196 = t193 * t195;
            let t197 = t190 * t196;
            let t198 = f64x8::splat(50.0) / f64x8::splat(81.0) * t197;
            let t199 = t56 * t9;
            let t201 = f64x8::splat(1.0) / t33 / t199;
            let t202 = t30 * t201;
            let t204 = f64x8::splat(7.0) / f64x8::splat(12.0) * t202 * t63;
            let t205 = t60 * t60;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t206 * t62;
            let t209 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t162 + f64x8::splat(5.0) / f64x8::splat(3.0) * t169 + f64x8::splat(5.0) / f64x8::splat(3.0) * t173;
            let t210 = t207 * t209;
            let t213 = t61 * t8;
            let t215 = t59 * t213 / f64x8::splat(4.0);
            let t216 = t204 + t59 * t210 / f64x8::splat(8.0) - t215;
            let t217 = t216 * t146;
            let t218 = t55 * t217;
            let t219 = t46 * t218;
            let t220 = f64x8::splat(5.0) / f64x8::splat(9.0) * t219;
            let t222 = f64x8::splat(1.0) / t32 / t9;
            let t223 = t72 * t222;
            let t226 = f64x8::splat(0.0011073470983333333) * t70 * t223 * t94;
            let t227 = t90 * t90;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t77 * t228;
            let t231 = f64x8::splat(1.0) / t78 * t67;
            let t232 = t69 * t72;
            let t233 = t232 * t222;
            let t234 = t231 * t233;
            let t236 = t70 * t223;
            let t238 = ((t75).sqrt());
            let t239 = t238 * t67;
            let t240 = t239 * t233;
            let t245 = t85 * t71 / t33 / t9;
            let t247 = -f64x8::splat(0.632975) * t234 - f64x8::splat(0.29896666666666666) * t236 - f64x8::splat(0.1023875) * t240 - f64x8::splat(0.08215666666666667) * t245;
            let t248 = f64x8::splat(1.0) / t93;
            let t249 = t247 * t248;
            let t251 = f64x8::splat(1.0) * t229 * t249;
            let t252 = t62 * t8;
            let t253 = t252 * t98;
            let t255 = f64x8::splat(4.0) * t253 * t142;
            let t256 = f64x8::splat(1.0) / t199;
            let t257 = t97 * t256;
            let t259 = f64x8::splat(4.0) * t257 * t142;
            let t262 = ((t100).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t103 * t166));
            let t263 = -t166;
            let t266 = ((t106).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t107 * t263));
            let t268 = (t262 + t266) * t113;
            let t269 = t268 * t141;
            let t274 = t121 * t121;
            let t275 = f64x8::splat(1.0) / t274;
            let t276 = t116 * t275;
            let t281 = -f64x8::splat(1.176575) * t234 - f64x8::splat(0.516475) * t236 - f64x8::splat(0.2103875) * t240 - f64x8::splat(0.104195) * t245;
            let t282 = f64x8::splat(1.0) / t124;
            let t283 = t281 * t282;
            let t289 = t134 * t134;
            let t290 = f64x8::splat(1.0) / t289;
            let t291 = t129 * t290;
            let t296 = -f64x8::splat(0.8630833333333333) * t234 - f64x8::splat(0.301925) * t236 - f64x8::splat(0.05501625) * t240 - f64x8::splat(0.082785) * t245;
            let t297 = f64x8::splat(1.0) / t137;
            let t298 = t296 * t297;
            let t301 = f64x8::splat(0.0005323764196666666) * t70 * t223 * t125 + f64x8::splat(1.0) * t276 * t283 - t226 - t251 + f64x8::splat(0.00018311447306006544) * t70 * t223 * t138 + f64x8::splat(0.5848223622634646) * t291 * t298;
            let t302 = t114 * t301;
            let t303 = t99 * t302;
            let t306 = t114 * t67;
            let t308 = t232 * t222 * t138;
            let t310 = f64x8::splat(0.00018311447306006544) * t306 * t308;
            let t311 = t114 * t129;
            let t313 = t290 * t296 * t297;
            let t315 = f64x8::splat(0.5848223622634646) * t311 * t313;
            let t316 = t226 + t251 + t255 - t259 + t99 * t269 + t303 + f64x8::splat(0.0197516734986138) * t268 * t139 - t310 - t315;
            let t317 = t66 * t316;
            let t318 = t55 * t317;
            let t319 = t46 * t318;
            let t320 = f64x8::splat(5.0) / f64x8::splat(9.0) * t319;
            let t321 = t181 * t40;
            let t322 = t321 * t153;
            let t324 = t39 * t188;
            let t325 = t47 * t192;
            let t326 = param_c * t180;
            let t327 = t325 * t326;
            let t330 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t322 + f64x8::splat(50.0) / f64x8::splat(81.0) * t324 * t327;
            let t331 = t330 * t146;
            let t332 = t156 * t316;
            let tvrho0 = t150 + t157 + t9 * (t184 - t198 + t220 + t320 + t331 + t332);
            acc_vrho_0 = tvrho0;
            let t335 = -t10 - t165;
            let t336 = t335 / f64x8::splat(2.0);
            let t337 = t15 * t336;
            let t338 = t7 * t337;
            let t340 = v_rho1 * v_rho1;
            let t342 = f64x8::splat(1.0) / t19 / t340;
            let t343 = v_tau1 * t342;
            let t344 = t343 * t27;
            let t346 = -t336;
            let t347 = t26 * t346;
            let t348 = t22 * t347;
            let t350 = f64x8::splat(5.0) / f64x8::splat(3.0) * t338 - f64x8::splat(5.0) / f64x8::splat(3.0) * t344 + f64x8::splat(5.0) / f64x8::splat(3.0) * t348 + t179;
            let t351 = t2 * t350;
            let t352 = t351 * t45;
            let t353 = t352 * t148;
            let t354 = f64x8::splat(5.0) / f64x8::splat(9.0) * t353;
            let t355 = t194 * t350;
            let t356 = t193 * t355;
            let t357 = t190 * t356;
            let t358 = f64x8::splat(50.0) / f64x8::splat(81.0) * t357;
            let t360 = f64x8::splat(5.0) / f64x8::splat(3.0) * t338 - f64x8::splat(5.0) / f64x8::splat(3.0) * t344 + f64x8::splat(5.0) / f64x8::splat(3.0) * t348;
            let t361 = t207 * t360;
            let t364 = t204 + t59 * t361 / f64x8::splat(8.0) + t215;
            let t365 = t364 * t146;
            let t366 = t55 * t365;
            let t367 = t46 * t366;
            let t368 = f64x8::splat(5.0) / f64x8::splat(9.0) * t367;
            let t371 = ((t100).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t103 * t335));
            let t372 = -t335;
            let t375 = ((t106).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t107 * t372));
            let t377 = (t371 + t375) * t113;
            let t378 = t377 * t141;
            let t382 = t226 + t251 - t255 - t259 + t99 * t378 + t303 + f64x8::splat(0.0197516734986138) * t377 * t139 - t310 - t315;
            let t383 = t66 * t382;
            let t384 = t55 * t383;
            let t385 = t46 * t384;
            let t386 = f64x8::splat(5.0) / f64x8::splat(9.0) * t385;
            let t387 = t351 * t40;
            let t388 = t387 * t153;
            let t390 = param_c * t350;
            let t391 = t325 * t390;
            let t394 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t388 + f64x8::splat(50.0) / f64x8::splat(81.0) * t324 * t391;
            let t395 = t394 * t146;
            let t396 = t156 * t382;
            let tvrho1 = t150 + t157 + t9 * (t354 - t358 + t368 + t386 + t395 + t396);
            acc_vrho_1 = tvrho1;
            let t399 = t2 * t35;
            let t400 = t399 * t45;
            let t401 = t400 * t148;
            let t402 = f64x8::splat(5.0) / f64x8::splat(72.0) * t401;
            let t403 = t194 * t35;
            let t405 = t190 * t193 * t403;
            let t406 = f64x8::splat(25.0) / f64x8::splat(324.0) * t405;
            let t407 = t39 * t50;
            let t408 = t54 * t58;
            let t409 = t63 * t146;
            let t410 = t408 * t409;
            let t411 = t407 * t410;
            let t412 = f64x8::splat(5.0) / f64x8::splat(72.0) * t411;
            let t414 = t399 * t40 * t153;
            let t416 = param_c * t35;
            let t418 = t324 * t325 * t416;
            let t420 = f64x8::splat(5.0) / f64x8::splat(72.0) * t414 - f64x8::splat(25.0) / f64x8::splat(324.0) * t418;
            let t421 = t420 * t146;
            let tvsigma0 = t9 * (-t402 + t406 - t412 + t421);
            acc_vsigma_0 = tvsigma0;
            let t423 = f64x8::splat(5.0) / f64x8::splat(36.0) * t401;
            let t424 = f64x8::splat(25.0) / f64x8::splat(162.0) * t405;
            let t425 = f64x8::splat(5.0) / f64x8::splat(36.0) * t411;
            let t428 = f64x8::splat(5.0) / f64x8::splat(36.0) * t414 - f64x8::splat(25.0) / f64x8::splat(162.0) * t418;
            let t429 = t428 * t146;
            let tvsigma1 = t9 * (-t423 + t424 - t425 + t429);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t431 = t2 * t6;
            let t432 = t16 * t40;
            let t433 = t431 * t432;
            let t434 = t54 * t66;
            let t435 = t434 * t146;
            let t436 = t152 * t435;
            let t438 = f64x8::splat(5.0) / f64x8::splat(9.0) * t433 * t436;
            let t439 = t193 * t146;
            let t440 = param_c * t6;
            let t441 = t440 * t16;
            let t442 = t439 * t441;
            let t444 = f64x8::splat(50.0) / f64x8::splat(81.0) * t190 * t442;
            let t445 = t59 * t206;
            let t446 = t62 * t6;
            let t447 = t16 * t146;
            let t448 = t446 * t447;
            let t449 = t445 * t448;
            let t451 = f64x8::splat(5.0) / f64x8::splat(72.0) * t154 * t449;
            let t452 = t431 * t16;
            let t453 = t45 * t55;
            let t456 = t325 * t441;
            let t459 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t452 * t453 + f64x8::splat(50.0) / f64x8::splat(81.0) * t324 * t456;
            let t460 = t459 * t146;
            let tvtau0 = t9 * (t438 - t444 + t451 + t460);
            acc_vtau_0 = tvtau0;
            let t462 = t2 * t21;
            let t463 = t27 * t40;
            let t464 = t462 * t463;
            let t466 = f64x8::splat(5.0) / f64x8::splat(9.0) * t464 * t436;
            let t467 = param_c * t21;
            let t468 = t467 * t27;
            let t471 = f64x8::splat(50.0) / f64x8::splat(81.0) * t190 * t439 * t468;
            let t472 = t62 * t21;
            let t473 = t27 * t146;
            let t474 = t472 * t473;
            let t475 = t445 * t474;
            let t477 = f64x8::splat(5.0) / f64x8::splat(72.0) * t154 * t475;
            let t478 = t462 * t27;
            let t479 = t478 * t453;
            let t484 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t479 + f64x8::splat(50.0) / f64x8::splat(81.0) * t324 * t325 * t468;
            let t485 = t484 * t146;
            let tvtau1 = t9 * (t466 - t471 + t477 + t485);
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
