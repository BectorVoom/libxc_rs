//! GGA_C_REVTCA kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 35 shared lines across all orders.
//! Delta: 85 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_revtca_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (35 lines) ---
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(1.0 <= zeta_threshold, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = M_CBRT3;
        let t9 = pow_1_3(1.0 / M_PI);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t18 = 0.488827e1 + 0.79425925e0 * t10 * t12 / t13;
        let t19 = f64::atan(t18);
        let t21 = -0.655868e0 * t19 + 0.897889e0;
        let t22 = t6 * t21;
        let t23 = t7 * t7;
        let t24 = t22 * t23;
        let t25 = 1.0 / t9;
        let t26 = t25 * t11;
        let t27 = M_CBRT6;
        let t28 = t27 * t27;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT2;
        let t34 = f64::sqrt(sigma[ip]);
        let t35 = t33 * t34;
        let t37 = 1.0 / t13 / rho[ip];
        let t39 = t32 * t35 * t37;
        let t40 = f64::powf(t39, 0.23e1);
        let t42 = 1.0 + 0.47121507034422759993e-2 * t40;
        let t43 = 1.0 / t42;
        let t46 = t24 * t26 * t13 * t43;
        let tzk0 = t46 / 3.0;
        zk[ip] += tzk0;
        // --- vxc delta (20 lines) ---
        let t48 = t18 * t18;
        let t49 = t48 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t6 * t50;
        let t55 = 1.0 / rho[ip] * t6;
        let t57 = t23 * t25;
        let t58 = t57 * t11;
        let t60 = t42 * t42;
        let t61 = 1.0 / t60;
        let t62 = f64::powf(t39, 0.13e1);
        let t63 = t61 * t62;
        let t64 = t63 * t28;
        let t65 = t31 * t33;
        let t66 = t65 * t34;
        let t67 = t64 * t66;
        let tvrho0 = 4.0 / 9.0 * t46 + 0.69457230103866666663e0 * t51 * t43 + 0.48168651635187710217e-2 * t55 * t21 * t58 * t67;
        vrho[ip] += tvrho0;
        let t70 = t22 * t58;
        let t71 = 1.0 / t34;
        let t72 = t65 * t71;
        let tvsigma0 = -0.18063244363195391331e-2 * t70 * t64 * t72;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (47 lines) ---
        let t76 = t50 * t43;
        let t79 = t13 * t13;
        let t85 = rho[ip] * rho[ip];
        let t86 = 1.0 / t85;
        let t91 = t49 * t49;
        let t92 = 1.0 / t91;
        let t93 = t6 * t92;
        let t94 = t43 * t18;
        let t102 = 1.0 / t13 / t85;
        let t107 = t85 * rho[ip];
        let t109 = 1.0 / t13 / t107;
        let t110 = t109 * t6;
        let t112 = t110 * t21 * t58;
        let t114 = 1.0 / t60 / t42;
        let t115 = f64::powf(t39, 0.26e1);
        let t116 = t114 * t115;
        let t117 = t116 * t27;
        let t118 = t30 * t30;
        let t119 = 1.0 / t118;
        let t120 = t33 * t33;
        let t121 = t119 * t120;
        let t122 = t121 * sigma[ip];
        let t123 = t117 * t122;
        let t126 = f64::powf(t39, 0.3e0);
        let t127 = t61 * t126;
        let t128 = t127 * t27;
        let t129 = t128 * t122;
        let tv2rho20 = 0.92609640138488888884e0 * t55 * t76 + 4.0 / 27.0 * t24 * t26 / t79 * t43 + 0.16056217211729236738e-2 * t24 * t26 * t86 * t67 + 0.36778031659583040509e0 * t93 * t94 * t10 * t12 * t37 + 0.20073966722509356988e-1 * t51 * t63 * t32 * t35 * t102 + 0.83527884012674580095e-3 * t112 * t123 - 0.50095397700595218626e-1 * t112 * t129;
        v2rho2[ip] += tv2rho20;
        let t133 = t50 * t61;
        let t134 = t6 * t37 * t133;
        let t135 = t62 * t28;
        let t136 = t135 * t72;
        let t139 = t121 * t102;
        let tv2rhosigma0 = -0.37638687604705044352e-2 * t134 * t136 - 0.31322956504752967535e-3 * t70 * t117 * t139 + 0.18785774137723206984e-1 * t70 * t128 * t139;
        v2rhosigma[ip] += tv2rhosigma0;
        let t147 = t24 * t26 * t114;
        let t148 = t115 * t27;
        let t149 = t148 * t119;
        let t150 = 1.0 / sigma[ip];
        let t151 = t120 * t150;
        let t152 = t151 * t37;
        let t157 = t24 * t26 * t61;
        let t158 = t126 * t27;
        let t159 = t158 * t119;
        let t163 = t34 * sigma[ip];
        let t164 = 1.0 / t163;
        let t165 = t65 * t164;
        let tv2sigma20 = 0.11746108689282362825e-3 * t147 * t149 * t152 - 0.70446653016462026191e-2 * t157 * t159 * t152 + 0.90316221815976956655e-3 * t70 * t64 * t165;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (85 lines) ---
        let t169 = t6 * t102;
        let t170 = t92 * t43;
        let t172 = t18 * t7;
        let t173 = t9 * t12;
        let t174 = t172 * t173;
        let t184 = 1.0 / t91 / t49;
        let t185 = t6 * t184;
        let t188 = t9 * t9;
        let t189 = t23 * t188;
        let t191 = 1.0 / t79 / t85;
        let t193 = t189 * t11 * t191;
        let t199 = t85 * t85;
        let t200 = t199 * rho[ip];
        let t202 = 1.0 / t79 / t200;
        let t203 = t202 * t6;
        let t204 = t21 * t23;
        let t205 = t204 * t25;
        let t206 = t203 * t205;
        let t207 = t11 * t114;
        let t208 = f64::powf(t39, 0.16e1);
        let t209 = 1.0 / t29;
        let t210 = t208 * t209;
        let t212 = t207 * t210 * t163;
        let t216 = f64::powf(t39, -0.7e0);
        let t217 = t216 * t209;
        let t219 = t11 * t61 * t217 * t163;
        let t225 = t110 * t133;
        let t226 = t135 * t66;
        let t231 = t120 * sigma[ip];
        let t233 = 1.0 / t79 / t199;
        let t235 = t27 * t119 * t231 * t233;
        let t241 = t203 * t204;
        let t242 = t60 * t60;
        let t243 = 1.0 / t242;
        let t244 = f64::powf(t39, 0.39e1);
        let t245 = t243 * t244;
        let t247 = t26 * t245 * t163;
        let t250 = t114 * t208;
        let t251 = t250 * t163;
        let t252 = t26 * t251;
        let t255 = t93 * t61;
        let t259 = 1.0 / t79 / t107;
        let t262 = t32 * t35;
        let t266 = 1.0 / t107;
        let t272 = 1.0 / t13 / t199;
        let t274 = t24 * t26 * t272;
        let tv3rho30 = -0.1e-19 * t169 * t170 * t174 - 8.0 / 81.0 * t24 * t26 / t79 / rho[ip] * t43 + 0.15579355649288896569e1 * t185 * t43 * t48 * t193 - 0.61739760092325925923e0 * t6 * t86 * t76 - 0.3474759974927262532e-1 * t206 * t212 + 0.2404579089628570494e0 * t206 * t219 - 0.38948389123222241422e0 * t93 * t43 * t193 - 0.30110950083764035481e-1 * t225 * t226 + 0.52214539139616816995e-2 * t51 * t116 * t235 - 0.31315388087114596902e0 * t51 * t127 * t235 + 0.44027089779786359603e-4 * t241 * t247 - 0.17603339676632507864e-2 * t241 * t252 + 0.15943933753545239999e-1 * t255 * t172 * t9 * t12 * t259 * t62 * t262 - 0.10704144807819491158e-2 * t24 * t26 * t266 * t67 - 0.25058365203802374029e-2 * t274 * t123 + 0.15028619310178565588e0 * t274 * t129;
        v3rho3[ip] += tv3rho30;
        let t279 = t169 * t133;
        let t282 = t6 * t191;
        let t284 = t282 * t92 * t64;
        let t285 = t72 * t174;
        let t288 = t6 * t259;
        let t289 = t50 * t114;
        let t290 = t288 * t289;
        let t291 = t148 * t121;
        let t294 = t288 * t133;
        let t295 = t158 * t121;
        let t298 = t22 * t57;
        let t299 = t11 * t243;
        let t306 = t209 * t233 * t34;
        let t310 = t121 * t109;
        let t319 = t61 * t216;
        let tv3rho2sigma0 = 0.50184916806273392469e-2 * t279 * t136 - 0.19929917191931549999e-2 * t284 * t285 - 0.13053634784904204249e-2 * t290 * t291 + 0.78288470217786492252e-1 * t294 * t295 - 0.16510158667419884851e-4 * t298 * t299 * t244 * t233 * t34 + 0.13030349905977234495e-1 * t70 * t250 * t306 + 0.73086898511090257582e-3 * t70 * t117 * t310 + 0.66012523787371904487e-3 * t298 * t207 * t208 * t233 * t34 - 0.90171715861071393523e-1 * t70 * t319 * t306 - 0.43833472988020816296e-1 * t70 * t128 * t310;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t326 = t282 * t289;
        let t327 = t121 * t150;
        let t328 = t148 * t327;
        let t331 = t244 * t71;
        let t336 = t209 * t71;
        let t337 = t336 * t259;
        let t345 = t282 * t133;
        let t346 = t158 * t327;
        let t349 = t208 * t71;
        let t357 = t135 * t165;
        let tv3rhosigma20 = 0.24475565221695382965e-3 * t326 * t328 + 0.61913095002824568187e-5 * t298 * t299 * t331 * t259 - 0.48863812147414629352e-2 * t70 * t250 * t337 + 0.1e-22 * t147 * t149 * t151 * t102 - 0.14679088165834967297e-1 * t345 * t346 - 0.24754696420264464183e-3 * t298 * t207 * t349 * t259 + 0.33814393447901772572e-1 * t70 * t319 * t337 + 0.18819343802352522176e-2 * t134 * t357;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t360 = t244 * t164;
        let t365 = t209 * t164;
        let t366 = t365 * t191;
        let t370 = sigma[ip] * sigma[ip];
        let t371 = 1.0 / t370;
        let t372 = t120 * t371;
        let t373 = t372 * t37;
        let t377 = t208 * t164;
        let t389 = 1.0 / t34 / t370;
        let t390 = t65 * t389;
        let tv3sigma30 = -0.23217410626059213069e-5 * t298 * t299 * t360 * t191 + 0.18323929555280486007e-2 * t70 * t250 * t366 - 0.17619163033923544238e-3 * t147 * t149 * t373 + 0.92830111575991740684e-4 * t298 * t207 * t377 * t191 - 0.12680397542963164714e-1 * t70 * t319 * t366 + 0.10566997952469303929e-1 * t157 * t159 * t373 - 0.13547433272396543498e-2 * t70 * t64 * t390;
        v3sigma3[ip] += tv3sigma30;
    }
}
