//! GGA_X_LG93 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lg93_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t83 = 1.0 + 0.20588079936467259283e0 * t34 + 0.1034375e0 * t39 * t41 * t45 + 0.39953563229732420473e-3 * t49 * t51 + 0.87666377314814814812e-3 * t57 * t59 * t62 + 0.9464819637345679012e-2 * t69 * t71 * t74 + 0.17770905884280507538e-7 * t78 * t80;
        let t84 = f64::powf(t83, 0.24974e-1);
        let t87 = 1.0 + 0.41666666666666666666e-9 * t34;
        let t88 = 1.0 / t87;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t84 * t88);
        let tzk0 = 2.0 * t92;
        zk[ip] += tzk0;
        let t93 = 1.0 / t30;
        let t98 = f64::powf(t83, -0.975026e0);
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
        let t132 = -0.54901546497246024755e0 * t25 * t28 * t102 - 0.55166666666666666667e0 * t39 * t41 * t108 - 0.31962850583785936378e-2 * t49 * t113 - 0.93510802469135802466e-2 * t57 * t59 * t118 - 0.12619759516460905349e0 * t69 * t71 * t124 - 0.28433449414848812061e-6 * t78 * t129;
        let t133 = t88 * t132;
        let t137 = t3 * t17;
        let t139 = 1.0 / t19 / t100;
        let t141 = t137 * t139 * t84;
        let t142 = t87 * t87;
        let t143 = 1.0 / t142;
        let t144 = t143 * t20;
        let t146 = t24 * sigma[ip] * t27;
        let t147 = t144 * t146;
        let t151 = piecewise3(t2, 0.0, -t18 * t93 * t84 * t88 / 8.0 - 0.936525e-2 * t18 * t99 * t133 - 0.28449335968970653394e-9 * t141 * t147);
        let tvrho0 = 2.0 * rho[ip] * t151 + 2.0 * t92;
        vrho[ip] += tvrho0;
        let t157 = sigma[ip] * t26;
        let t163 = t49 * t27;
        let t167 = t58 * t26;
        let t173 = 0.20588079936467259283e0 * t25 * t27 * t32 + 0.206875e0 * t39 * t157 * t45 + 0.11986068968919726142e-2 * t40 * t51 + 0.35066550925925925925e-2 * t57 * t163 * t62 + 0.4732409818672839506e-1 * t69 * t167 * t74 + 0.10662543530568304523e-6 * t70 * t80;
        let t174 = t88 * t173;
        let t179 = 1.0 / t19 / t29;
        let t182 = t24 * t27;
        let t183 = t144 * t182;
        let t187 = piecewise3(t2, 0.0, -0.936525e-2 * t18 * t99 * t174 + 0.10668500988363995023e-9 * t137 * t179 * t84 * t183);
        let tvsigma0 = 2.0 * rho[ip] * t187;
        vsigma[ip] += tvsigma0;
        let t191 = 1.0 / t30 / rho[ip];
        let t196 = t93 * t98;
        let t201 = 1.0 / t19 / t42;
        let t203 = t137 * t201 * t84;
        let t206 = f64::powf(t83, -0.1975026e1);
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
        let t250 = 0.20130567048990209077e1 * t25 * t28 * t221 + 0.34938888888888888889e1 * t39 * t41 * t227 + 0.2876656552540734274e-1 * t49 * t231 + 0.10909593621399176954e0 * t57 * t59 * t236 + 0.18088321973593964334e1 * t69 * t71 * t242 + 0.48336864005242980504e-5 * t78 * t247;
        let t251 = t88 * t250;
        let t255 = 1.0 / t225;
        let t257 = t137 * t255 * t84;
        let t259 = 1.0 / t142 / t87;
        let t260 = t259 * t36;
        let t262 = t38 * t40 * t26;
        let t263 = t260 * t262;
        let t267 = piecewise3(t2, 0.0, t18 * t191 * t84 * t88 / 12.0 - 0.62435e-2 * t18 * t196 * t133 + 0.85348007906911960182e-9 * t203 * t147 + 0.91313622465e-2 * t18 * t207 * t209 - 0.14209874329781461957e-10 * t215 * t217 - 0.936525e-2 * t18 * t99 * t251 - 0.12644149319542512619e-17 * t257 * t263);
        let tv2rho20 = 2.0 * rho[ip] * t267 + 4.0 * t151;
        v2rho2[ip] += tv2rho20;
        let t274 = t6 * t17 * t19;
        let t275 = t206 * t88;
        let t276 = t173 * t132;
        let t277 = t275 * t276;
        let t280 = t173 * t20;
        let t281 = t280 * t146;
        let t300 = -0.54901546497246024755e0 * t25 * t27 * t102 - 0.11033333333333333333e1 * t39 * t157 * t108 - 0.95888551751357809136e-2 * t40 * t113 - 0.37404320987654320987e-1 * t57 * t163 * t118 - 0.63098797582304526747e0 * t69 * t167 * t124 - 0.17060069648909287237e-5 * t70 * t129;
        let t301 = t88 * t300;
        let t308 = t137 * t179 * t98;
        let t309 = t182 * t132;
        let t310 = t144 * t309;
        let t313 = 1.0 / t106;
        let t315 = t137 * t313 * t84;
        let t316 = t38 * t26;
        let t317 = t316 * sigma[ip];
        let t318 = t260 * t317;
        let t322 = piecewise3(t2, 0.0, -0.312175e-2 * t18 * t196 * t174 + 0.91313622465e-2 * t274 * t277 - 0.71049371648907309786e-11 * t215 * t281 - 0.936525e-2 * t18 * t99 * t301 - 0.2489316897284932172e-9 * t141 * t183 + 0.2664351436834024117e-11 * t308 * t310 + 0.47415559948284422324e-18 * t315 * t318);
        let tv2rhosigma0 = 2.0 * rho[ip] * t322 + 2.0 * t187;
        v2rhosigma[ip] += tv2rhosigma0;
        let t325 = t173 * t173;
        let t326 = t88 * t325;
        let t331 = t25 * t27;
        let t332 = t143 * t173 * t331;
        let t340 = t40 * t27;
        let t344 = t49 * t26;
        let t350 = 0.206875e0 * t39 * t26 * t45 + 0.23972137937839452284e-2 * sigma[ip] * t51 + 0.10519965277777777778e-1 * t57 * t340 * t62 + 0.18929639274691358024e0 * t69 * t344 * t74 + 0.53312717652841522615e-6 * t58 * t80;
        let t351 = t88 * t350;
        let t355 = 1.0 / t43;
        let t358 = t260 * t316;
        let t362 = piecewise3(t2, 0.0, 0.91313622465e-2 * t18 * t207 * t326 + 0.5328702873668048234e-11 * t308 * t332 - 0.936525e-2 * t18 * t99 * t351 - 0.17780834980606658371e-18 * t137 * t355 * t84 * t358);
        let tv2sigma20 = 2.0 * rho[ip] * t362;
        v2sigma2[ip] += tv2sigma20;
    }
}
