//! GGA_X_LG93 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lg93_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t19 * t19;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = t20 * t20;
        let t38 = 1.0 / t22 / t21;
        let t39 = t36 * t38;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t40 * t26;
        let t42 = t29 * t29;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t19 / t43;
        let t49 = t40 * sigma[ip];
        let t50 = t42 * t42;
        let t51 = 1.0 / t50;
        let t54 = t21 * t21;
        let t57 = t20 / t23 / t54;
        let t58 = t40 * t40;
        let t59 = t58 * t27;
        let t60 = t50 * t29;
        let t62 = 1.0 / t30 / t60;
        let t69 = t36 / t22 / t54 / t21;
        let t70 = t58 * sigma[ip];
        let t71 = t70 * t26;
        let t72 = t50 * t43;
        let t74 = 1.0 / t19 / t72;
        let t78 = t58 * t40;
        let t79 = t50 * t50;
        let t80 = 1.0 / t79;
        let t83 = 1.0 + 0.2058807993646726 * t34 + 0.1034375 * t39 * t41 * t45 + 0.0003995356322973242 * t49 * t51 + 0.0008766637731481481 * t57 * t59 * t62 + 0.009464819637345679 * t69 * t71 * t74 + 1.7770905884280507e-08 * t78 * t80;
        let t84 = rmath::pow(t83, 0.024974);
        let t87 = 1.0 + 4.166666666666667e-10 * t34;
        let t88 = 1.0 / t87;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t84 * t88);
        let tzk0 = 2.0 * t92;
        zk[ip] += tzk0;
        let t93 = 1.0 / t30;
        let t98 = rmath::pow(t83, -0.975026);
        let t99 = t19 * t98;
        let t100 = t29 * rho[ip];
        let t102 = 1.0 / t30 / t100;
        let t106 = t42 * t29;
        let t108 = 1.0 / t19 / t106;
        let t112 = t50 * rho[ip];
        let t113 = 1.0 / t112;
        let t116 = t50 * t100;
        let t118 = 1.0 / t30 / t116;
        let t122 = t50 * t106;
        let t124 = 1.0 / t19 / t122;
        let t128 = t79 * rho[ip];
        let t129 = 1.0 / t128;
        let t132 = -0.5490154649724602 * t25 * t28 * t102 - 0.5516666666666666 * t39 * t41 * t108 - 0.0031962850583785937 * t49 * t113 - 0.009351080246913581 * t57 * t59 * t118 - 0.12619759516460904 * t69 * t71 * t124 - 2.843344941484881e-07 * t78 * t129;
        let t133 = t88 * t132;
        let t137 = t3 * t17;
        let t139 = 1.0 / t19 / t100;
        let t141 = t137 * t139 * t84;
        let t142 = t87 * t87;
        let t143 = 1.0 / t142;
        let t144 = t143 * t20;
        let t146 = t24 * sigma[ip] * t27;
        let t147 = t144 * t146;
        let t151 = piecewise3(t2, 0.0, -t18 * t93 * t84 * t88 / 8.0 - 0.00936525 * t18 * t99 * t133 - 2.8449335968970655e-10 * t141 * t147);
        let tvrho0 = 2.0 * rho[ip] * t151 + 2.0 * t92;
        vrho[ip] += tvrho0;
        let t157 = sigma[ip] * t26;
        let t163 = t49 * t27;
        let t167 = t58 * t26;
        let t173 = 0.2058807993646726 * t25 * t27 * t32 + 0.206875 * t39 * t157 * t45 + 0.0011986068968919726 * t40 * t51 + 0.0035066550925925925 * t57 * t163 * t62 + 0.04732409818672839 * t69 * t167 * t74 + 1.0662543530568304e-07 * t70 * t80;
        let t174 = t88 * t173;
        let t179 = 1.0 / t19 / t29;
        let t182 = t24 * t27;
        let t183 = t144 * t182;
        let t187 = piecewise3(t2, 0.0, -0.00936525 * t18 * t99 * t174 + 1.0668500988363994e-10 * t137 * t179 * t84 * t183);
        let tvsigma0 = 2.0 * rho[ip] * t187;
        vsigma[ip] += tvsigma0;
        let t191 = 1.0 / t30 / rho[ip];
        let t196 = t93 * t98;
        let t201 = 1.0 / t19 / t42;
        let t203 = t137 * t201 * t84;
        let t206 = rmath::pow(t83, -1.975026);
        let t207 = t19 * t206;
        let t208 = t132 * t132;
        let t209 = t88 * t208;
        let t213 = t139 * t98;
        let t215 = t137 * t213 * t143;
        let t216 = t132 * t20;
        let t217 = t216 * t146;
        let t221 = 1.0 / t30 / t42;
        let t225 = t42 * t100;
        let t227 = 1.0 / t19 / t225;
        let t231 = 1.0 / t60;
        let t234 = t50 * t42;
        let t236 = 1.0 / t30 / t234;
        let t242 = 1.0 / t19 / t50 / t225;
        let t247 = 1.0 / t79 / t29;
        let t250 = 2.013056704899021 * t25 * t28 * t221 + 3.493888888888889 * t39 * t41 * t227 + 0.028766565525407344 * t49 * t231 + 0.10909593621399177 * t57 * t59 * t236 + 1.8088321973593964 * t69 * t71 * t242 + 4.833686400524298e-06 * t78 * t247;
        let t251 = t88 * t250;
        let t255 = 1.0 / t225;
        let t257 = t137 * t255 * t84;
        let t259 = 1.0 / t142 / t87;
        let t260 = t259 * t36;
        let t262 = t38 * t40 * t26;
        let t263 = t260 * t262;
        let t267 = piecewise3(t2, 0.0, t18 * t191 * t84 * t88 / 12.0 - 0.0062435 * t18 * t196 * t133 + 8.534800790691196e-10 * t203 * t147 + 0.0091313622465 * t18 * t207 * t209 - 1.4209874329781462e-11 * t215 * t217 - 0.00936525 * t18 * t99 * t251 - 1.2644149319542513e-18 * t257 * t263);
        let tv2rho20 = 2.0 * rho[ip] * t267 + 4.0 * t151;
        v2rho2[ip] += tv2rho20;
        let t274 = t6 * t17 * t19;
        let t275 = t206 * t88;
        let t276 = t173 * t132;
        let t277 = t275 * t276;
        let t280 = t173 * t20;
        let t281 = t280 * t146;
        let t300 = -0.5490154649724602 * t25 * t27 * t102 - 1.1033333333333333 * t39 * t157 * t108 - 0.009588855175135781 * t40 * t113 - 0.037404320987654324 * t57 * t163 * t118 - 0.6309879758230452 * t69 * t167 * t124 - 1.7060069648909286e-06 * t70 * t129;
        let t301 = t88 * t300;
        let t308 = t137 * t179 * t98;
        let t309 = t182 * t132;
        let t310 = t144 * t309;
        let t313 = 1.0 / t106;
        let t315 = t137 * t313 * t84;
        let t316 = t38 * t26;
        let t317 = t316 * sigma[ip];
        let t318 = t260 * t317;
        let t322 = piecewise3(t2, 0.0, -0.00312175 * t18 * t196 * t174 + 0.0091313622465 * t274 * t277 - 7.104937164890731e-12 * t215 * t281 - 0.00936525 * t18 * t99 * t301 - 2.4893168972849323e-10 * t141 * t183 + 2.664351436834024e-12 * t308 * t310 + 4.741555994828442e-19 * t315 * t318);
        let tv2rhosigma0 = 2.0 * rho[ip] * t322 + 2.0 * t187;
        v2rhosigma[ip] += tv2rhosigma0;
        let t325 = t173 * t173;
        let t326 = t88 * t325;
        let t331 = t25 * t27;
        let t332 = t143 * t173 * t331;
        let t340 = t40 * t27;
        let t344 = t49 * t26;
        let t350 = 0.206875 * t39 * t26 * t45 + 0.0023972137937839453 * sigma[ip] * t51 + 0.010519965277777777 * t57 * t340 * t62 + 0.18929639274691357 * t69 * t344 * t74 + 5.331271765284152e-07 * t58 * t80;
        let t351 = t88 * t350;
        let t355 = 1.0 / t43;
        let t358 = t260 * t316;
        let t362 = piecewise3(t2, 0.0, 0.0091313622465 * t18 * t207 * t326 + 5.328702873668048e-12 * t308 * t332 - 0.00936525 * t18 * t99 * t351 - 1.7780834980606658e-19 * t137 * t355 * t84 * t358);
        let tv2sigma20 = 2.0 * rho[ip] * t362;
        v2sigma2[ip] += tv2sigma20;
    }
}
