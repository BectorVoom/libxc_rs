//! GGA_C_CHACHIYO kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_chachiyo_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = pow_1_3(rho[ip]);
        let t10 = t8 * t9;
        let t13 = param_cp * t1;
        let t14 = t5 * t5;
        let t16 = t7 * t7;
        let t17 = 1.0 / t14 * t16;
        let t18 = t9 * t9;
        let t19 = t17 * t18;
        let t22 = 1.0 + t3 * t10 / 3.0 + t13 * t19 / 3.0;
        let t23 = rmath::ln(t22);
        let t24 = param_ap * t23;
        let t25 = param_bf * t2;
        let t28 = param_cf * t1;
        let t31 = 1.0 + t25 * t10 / 3.0 + t28 * t19 / 3.0;
        let t32 = rmath::ln(t31);
        let t36 = pow_1_3(zeta_threshold);
        let t37 = t36 * t36;
        let t38 = piecewise3(1.0 <= zeta_threshold, t37, 1.0);
        let t39 = t38 * t38;
        let t42 = -2.0 * t39 * t38 + 2.0;
        let t44 = t24 + (param_af * t32 - t24) * t42;
        let t45 = M_CBRTPI;
        let t46 = t2 * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t9 / t47;
        let t53 = 1.0 + t46 * t49 * sigma[ip] / 48.0;
        let t54 = 1.0 / t44;
        let t55 = param_h * t54;
        let t56 = rmath::pow(t53, t55);
        let tzk0 = t44 * t56;
        zk[ip] += tzk0;
        let t58 = t8 / t18;
        let t62 = t17 / t9;
        let t65 = t3 * t58 / 9.0 + 2.0 / 9.0 * t13 * t62;
        let t67 = 1.0 / t22;
        let t68 = param_ap * t65 * t67;
        let t73 = t25 * t58 / 9.0 + 2.0 / 9.0 * t28 * t62;
        let t75 = 1.0 / t31;
        let t79 = t68 + (param_af * t73 * t75 - t68) * t42;
        let t80 = rho[ip] * t79;
        let t82 = rho[ip] * t44;
        let t83 = t44 * t44;
        let t84 = 1.0 / t83;
        let t85 = param_h * t84;
        let t86 = rmath::ln(t53);
        let t87 = t79 * t86;
        let t89 = t55 * t2;
        let t90 = t47 * rho[ip];
        let t92 = 1.0 / t9 / t90;
        let t93 = t45 * t92;
        let t94 = 1.0 / t53;
        let t95 = sigma[ip] * t94;
        let t96 = t93 * t95;
        let t99 = -t85 * t87 - 7.0 / 144.0 * t89 * t96;
        let t100 = t56 * t99;
        let tvrho0 = t82 * t100 + t80 * t56 + tzk0;
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t9 / rho[ip];
        let t104 = t103 * t56;
        let t106 = t46 * t94;
        let tvsigma0 = t104 * param_h * t106 / 48.0;
        vsigma[ip] += tvsigma0;
        let t108 = t79 * t56;
        let t110 = t44 * t56;
        let t115 = t8 / t18 / rho[ip];
        let t117 = t17 * t103;
        let t120 = -2.0 / 27.0 * t3 * t115 - 2.0 / 27.0 * t13 * t117;
        let t121 = param_ap * t120;
        let t122 = t121 * t67;
        let t123 = t65 * t65;
        let t125 = t22 * t22;
        let t126 = 1.0 / t125;
        let t127 = param_ap * t123 * t126;
        let t131 = -2.0 / 27.0 * t25 * t115 - 2.0 / 27.0 * t28 * t117;
        let t132 = param_af * t131;
        let t134 = t73 * t73;
        let t136 = t31 * t31;
        let t137 = 1.0 / t136;
        let t141 = t122 - t127 + (-param_af * t134 * t137 + t132 * t75 - t122 + t127) * t42;
        let t142 = rho[ip] * t141;
        let t146 = t99 * t99;
        let t147 = t56 * t146;
        let t150 = 1.0 / t83 / t44;
        let t151 = param_h * t150;
        let t152 = t79 * t79;
        let t153 = t152 * t86;
        let t158 = t79 * t2;
        let t159 = t85 * t158;
        let t162 = t47 * t47;
        let t164 = 1.0 / t9 / t162;
        let t166 = t45 * t164 * t95;
        let t169 = t55 * t1;
        let t170 = t45 * t45;
        let t171 = t162 * t47;
        let t173 = 1.0 / t18 / t171;
        let t175 = sigma[ip] * sigma[ip];
        let t176 = t53 * t53;
        let t177 = 1.0 / t176;
        let t178 = t175 * t177;
        let t179 = t170 * t173 * t178;
        let t182 = 2.0 * t151 * t153 - t85 * t141 * t86 + 7.0 / 72.0 * t159 * t96 + 35.0 / 216.0 * t89 * t166 - 49.0 / 6912.0 * t169 * t179;
        let t183 = t56 * t182;
        let tv2rho20 = 2.0 * t80 * t100 + 2.0 * t110 * t99 + t142 * t56 + t82 * t147 + t82 * t183 + 2.0 * t108;
        v2rho2[ip] += tv2rho20;
        let t185 = t49 * t56;
        let t192 = param_h * t2 * t45 * t94;
        let t197 = 1.0 / t18 / t162 * t56;
        let t198 = t197 * param_h;
        let t199 = t1 * t170;
        let t201 = t199 * t177 * sigma[ip];
        let tv2rhosigma0 = -t185 * param_h * t106 / 36.0 + t104 * t99 * t192 / 48.0 + 7.0 / 2304.0 * t198 * t201;
        v2rhosigma[ip] += tv2rhosigma0;
        let t205 = 1.0 / t18 / t90;
        let t206 = t205 * t56;
        let t207 = param_h * param_h;
        let t210 = t170 * t177;
        let t211 = t54 * t1 * t210;
        let t214 = t199 * t177;
        let tv2sigma20 = t206 * t207 * t211 / 768.0 - t206 * param_h * t214 / 768.0;
        v2sigma2[ip] += tv2sigma20;
        let t217 = t141 * t56;
        let t227 = t8 / t18 / t47;
        let t230 = t17 * t49;
        let t234 = param_ap * (10.0 / 81.0 * t3 * t227 + 8.0 / 81.0 * t13 * t230);
        let t235 = t234 * t67;
        let t236 = t126 * t65;
        let t238 = 3.0 * t121 * t236;
        let t242 = 1.0 / t125 / t22;
        let t244 = 2.0 * param_ap * t123 * t65 * t242;
        let t250 = param_af * (10.0 / 81.0 * t25 * t227 + 8.0 / 81.0 * t28 * t230);
        let t252 = t137 * t73;
        let t258 = 1.0 / t136 / t31;
        let t263 = t235 - t238 + t244 + (2.0 * param_af * t134 * t73 * t258 - 3.0 * t132 * t252 + t250 * t75 - t235 + t238 - t244) * t42;
        let t264 = rho[ip] * t263;
        let t272 = t146 * t99;
        let t273 = t56 * t272;
        let t275 = t100 * t182;
        let t278 = t83 * t83;
        let t280 = param_h / t278;
        let t281 = t152 * t79;
        let t289 = t151 * t152 * t2;
        let t295 = t85 * t141 * t2;
        let t301 = t85 * t79 * t1;
        let t304 = t162 * rho[ip];
        let t308 = t45 / t9 / t304 * t95;
        let t311 = t162 * t90;
        let t315 = t170 / t18 / t311 * t178;
        let t318 = t55 * M_PI;
        let t319 = t162 * t162;
        let t320 = t319 * t47;
        let t321 = 1.0 / t320;
        let t322 = t175 * sigma[ip];
        let t325 = 1.0 / t176 / t53;
        let t329 = -6.0 * t280 * t281 * t86 + 6.0 * t151 * t87 * t141 - 7.0 / 24.0 * t289 * t96 - t85 * t263 * t86 + 7.0 / 48.0 * t295 * t96 - 35.0 / 72.0 * t159 * t166 + 49.0 / 2304.0 * t301 * t179 - 455.0 / 648.0 * t89 * t308 + 245.0 / 3456.0 * t169 * t315 - 343.0 / 165888.0 * t318 * t321 * t322 * t325;
        let t330 = t56 * t329;
        let tv3rho30 = 3.0 * t142 * t100 + 6.0 * t108 * t99 + 3.0 * t110 * t146 + 3.0 * t110 * t182 + 3.0 * t80 * t147 + 3.0 * t80 * t183 + t264 * t56 + t82 * t273 + 3.0 * t82 * t275 + t82 * t330 + 3.0 * t217;
        v3rho3[ip] += tv3rho30;
        let t332 = t92 * t56;
        let t341 = 1.0 / t18 / t304 * t56;
        let t342 = t341 * param_h;
        let t351 = t99 * param_h;
        let t356 = 1.0 / t319 * t56;
        let t357 = t356 * param_h;
        let t358 = M_PI * t325;
        let t359 = t358 * t175;
        let tv3rho2sigma0 = 7.0 / 108.0 * t332 * param_h * t106 - t185 * t99 * t192 / 18.0 - 7.0 / 384.0 * t342 * t201 + t104 * t146 * t192 / 48.0 + t104 * t182 * t192 / 48.0 + 7.0 / 1152.0 * t197 * t351 * t201 + 49.0 / 55296.0 * t357 * t359;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t365 = t99 * t207;
        let t366 = t206 * t365;
        let t369 = t207 * t84;
        let t370 = t206 * t369;
        let t372 = t199 * t177 * t79;
        let t376 = 1.0 / t311 * t56;
        let t377 = t376 * t207;
        let t378 = t54 * M_PI;
        let t379 = t325 * sigma[ip];
        let t380 = t378 * t379;
        let t387 = param_h * t1 * t210;
        let t391 = t358 * sigma[ip];
        let tv3rhosigma20 = -11.0 / 2304.0 * t197 * t207 * t211 + t366 * t211 / 768.0 - t370 * t372 / 768.0 + 7.0 / 18432.0 * t377 * t380 + 11.0 / 2304.0 * t198 * t214 - t206 * t99 * t387 / 768.0 - 7.0 / 18432.0 * t376 * param_h * t391;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t395 = 1.0 / t171 * t56;
        let t396 = t207 * param_h;
        let t397 = t395 * t396;
        let t398 = t84 * M_PI;
        let t399 = t398 * t325;
        let t402 = t395 * t207;
        let t403 = t378 * t325;
        let t406 = param_h * M_PI;
        let t407 = t406 * t325;
        let tv3sigma30 = t397 * t399 / 12288.0 - t402 * t403 / 4096.0 + t395 * t407 / 6144.0;
        v3sigma3[ip] += tv3sigma30;
    }
}
