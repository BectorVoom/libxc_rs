//! GGA_C_REVTCA kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_revtca_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t18 = 4.88827 + 0.79425925 * t10 * t12 / t13;
        let t19 = rmath::atan(t18);
        let t21 = -0.655868 * t19 + 0.897889;
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
        let t34 = rmath::sqrt(sigma[ip]);
        let t35 = t33 * t34;
        let t37 = 1.0 / t13 / rho[ip];
        let t39 = t32 * t35 * t37;
        let t40 = rmath::pow(t39, 2.3);
        let t42 = 1.0 + 0.004712150703442276 * t40;
        let t43 = 1.0 / t42;
        let t46 = t24 * t26 * t13 * t43;
        let tzk0 = t46 / 3.0;
        zk[ip] += tzk0;
        let t48 = t18 * t18;
        let t49 = t48 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t6 * t50;
        let t55 = 1.0 / rho[ip] * t6;
        let t57 = t23 * t25;
        let t58 = t57 * t11;
        let t60 = t42 * t42;
        let t61 = 1.0 / t60;
        let t62 = rmath::pow(t39, 1.3);
        let t63 = t61 * t62;
        let t64 = t63 * t28;
        let t65 = t31 * t33;
        let t66 = t65 * t34;
        let t67 = t64 * t66;
        let tvrho0 = 4.0 / 9.0 * t46 + 0.6945723010386666 * t51 * t43 + 0.004816865163518771 * t55 * t21 * t58 * t67;
        vrho[ip] += tvrho0;
        let t70 = t22 * t58;
        let t71 = 1.0 / t34;
        let t72 = t65 * t71;
        let tvsigma0 = -0.001806324436319539 * t70 * t64 * t72;
        vsigma[ip] += tvsigma0;
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
        let t115 = rmath::pow(t39, 2.6);
        let t116 = t114 * t115;
        let t117 = t116 * t27;
        let t118 = t30 * t30;
        let t119 = 1.0 / t118;
        let t120 = t33 * t33;
        let t121 = t119 * t120;
        let t122 = t121 * sigma[ip];
        let t123 = t117 * t122;
        let t126 = rmath::pow(t39, 0.3);
        let t127 = t61 * t126;
        let t128 = t127 * t27;
        let t129 = t128 * t122;
        let tv2rho20 = 0.9260964013848889 * t55 * t76 + 4.0 / 27.0 * t24 * t26 / t79 * t43 + 0.0016056217211729237 * t24 * t26 * t86 * t67 + 0.3677803165958304 * t93 * t94 * t10 * t12 * t37 + 0.020073966722509357 * t51 * t63 * t32 * t35 * t102 + 0.0008352788401267458 * t112 * t123 - 0.05009539770059522 * t112 * t129;
        v2rho2[ip] += tv2rho20;
        let t133 = t50 * t61;
        let t134 = t6 * t37 * t133;
        let t135 = t62 * t28;
        let t136 = t135 * t72;
        let t139 = t121 * t102;
        let tv2rhosigma0 = -0.0037638687604705044 * t134 * t136 - 0.0003132295650475297 * t70 * t117 * t139 + 0.018785774137723206 * t70 * t128 * t139;
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
        let tv2sigma20 = 0.00011746108689282363 * t147 * t149 * t152 - 0.007044665301646202 * t157 * t159 * t152 + 0.0009031622181597695 * t70 * t64 * t165;
        v2sigma2[ip] += tv2sigma20;
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
        let t208 = rmath::pow(t39, 1.6);
        let t209 = 1.0 / t29;
        let t210 = t208 * t209;
        let t212 = t207 * t210 * t163;
        let t216 = rmath::pow(t39, -0.7);
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
        let t244 = rmath::pow(t39, 3.9);
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
        let tv3rho30 = -1e-20 * t169 * t170 * t174 - 8.0 / 81.0 * t24 * t26 / t79 / rho[ip] * t43 + 1.5579355649288897 * t185 * t43 * t48 * t193 - 0.6173976009232592 * t6 * t86 * t76 - 0.03474759974927263 * t206 * t212 + 0.24045790896285704 * t206 * t219 - 0.38948389123222243 * t93 * t43 * t193 - 0.030110950083764035 * t225 * t226 + 0.0052214539139616815 * t51 * t116 * t235 - 0.313153880871146 * t51 * t127 * t235 + 4.402708977978636e-05 * t241 * t247 - 0.0017603339676632507 * t241 * t252 + 0.01594393375354524 * t255 * t172 * t9 * t12 * t259 * t62 * t262 - 0.0010704144807819492 * t24 * t26 * t266 * t67 - 0.0025058365203802376 * t274 * t123 + 0.15028619310178565 * t274 * t129;
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
        let tv3rho2sigma0 = 0.005018491680627339 * t279 * t136 - 0.001992991719193155 * t284 * t285 - 0.0013053634784904204 * t290 * t291 + 0.0782884702177865 * t294 * t295 - 1.6510158667419884e-05 * t298 * t299 * t244 * t233 * t34 + 0.013030349905977234 * t70 * t250 * t306 + 0.0007308689851109025 * t70 * t117 * t310 + 0.000660125237873719 * t298 * t207 * t208 * t233 * t34 - 0.0901717158610714 * t70 * t319 * t306 - 0.043833472988020816 * t70 * t128 * t310;
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
        let tv3rhosigma20 = 0.0002447556522169538 * t326 * t328 + 6.191309500282457e-06 * t298 * t299 * t331 * t259 - 0.004886381214741463 * t70 * t250 * t337 + 1e-23 * t147 * t149 * t151 * t102 - 0.014679088165834967 * t345 * t346 - 0.00024754696420264467 * t298 * t207 * t349 * t259 + 0.03381439344790177 * t70 * t319 * t337 + 0.0018819343802352522 * t134 * t357;
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
        let tv3sigma30 = -2.3217410626059214e-06 * t298 * t299 * t360 * t191 + 0.0018323929555280486 * t70 * t250 * t366 - 0.00017619163033923545 * t147 * t149 * t373 + 9.283011157599174e-05 * t298 * t207 * t377 * t191 - 0.012680397542963165 * t70 * t319 * t366 + 0.010566997952469305 * t157 * t159 * t373 - 0.0013547433272396543 * t70 * t64 * t390;
        v3sigma3[ip] += tv3sigma30;
    }
}
