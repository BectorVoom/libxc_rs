//! GGA_X_RGE2 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_rge2_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
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
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3::<f64>(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t36 = t20 * t20;
        let t38 = 1.0 / t22 / t21;
        let t39 = t36 * t38;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t40 * t26;
        let t42 = t29 * t29;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t49 = 0.804e0 + 5.0 / 972.0 * t25 * t28 * t32 + 0.65823568907145082055e-4 * t39 * t41 * t45;
        let t52 = 0.1804e1 - 0.646416e0 / t49;
        let t56 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        let t57 = 1.0 / t30;
        let t62 = t3 * t17;
        let t63 = t49 * t49;
        let t64 = 1.0 / t63;
        let t65 = t18 * t64;
        let t66 = t29 * rho[ip];
        let t68 = 1.0 / t30 / t66;
        let t72 = t42 * t29;
        let t74 = 1.0 / t18 / t72;
        let t78 = -10.0 / 729.0 * t25 * t28 * t68 - 0.35105903417144043763e-3 * t39 * t41 * t74;
        let t83 = piecewise3::<f64>(t2, 0.0, -t6 * t17 * t57 * t52 / 8.0 - 0.16551095363746320496e0 * t62 * t65 * t78);
        let tvrho0 = 2.0 * rho[ip] * t83 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t89 = sigma[ip] * t26;
        let t93 = 5.0 / 972.0 * t25 * t27 * t32 + 0.13164713781429016411e-3 * t39 * t89 * t45;
        let t97 = piecewise3::<f64>(t2, 0.0, -0.16551095363746320496e0 * t62 * t65 * t93);
        let tvsigma0 = 2.0 * rho[ip] * t97;
        vsigma[ip] += tvsigma0;
        let t101 = 1.0 / t30 / rho[ip];
        let t106 = t57 * t64;
        let t111 = 1.0 / t63 / t49;
        let t112 = t18 * t111;
        let t113 = t78 * t78;
        let t118 = 1.0 / t30 / t42;
        let t122 = t42 * t66;
        let t124 = 1.0 / t18 / t122;
        let t128 = 110.0 / 2187.0 * t25 * t28 * t118 + 0.22233738830857894383e-2 * t39 * t41 * t124;
        let t133 = piecewise3::<f64>(t2, 0.0, t6 * t17 * t101 * t52 / 12.0 - 0.11034063575830880331e0 * t62 * t106 * t78 + 0.33102190727492640992e0 * t62 * t112 * t113 - 0.16551095363746320496e0 * t62 * t65 * t128);
        let tv2rho20 = 2.0 * rho[ip] * t133 + 4.0 * t83;
        v2rho2[ip] += tv2rho20;
        let t139 = t62 * t18;
        let t140 = t111 * t93;
        let t141 = t140 * t78;
        let t150 = -10.0 / 729.0 * t25 * t27 * t68 - 0.70211806834288087525e-3 * t39 * t89 * t74;
        let t155 = piecewise3::<f64>(t2, 0.0, -0.55170317879154401653e-1 * t62 * t106 * t93 + 0.33102190727492640992e0 * t139 * t141 - 0.16551095363746320496e0 * t62 * t65 * t150);
        let tv2rhosigma0 = 2.0 * rho[ip] * t155 + 2.0 * t97;
        v2rhosigma[ip] += tv2rhosigma0;
        let t158 = t93 * t93;
        let t162 = 1.0 / t43;
        let t165 = t38 * t26;
        let t166 = t64 * t36 * t165;
        let t170 = piecewise3::<f64>(t2, 0.0, 0.33102190727492640992e0 * t62 * t112 * t158 - 0.21789043323285708475e-4 * t62 * t162 * t166);
        let tv2sigma20 = 2.0 * rho[ip] * t170;
        v2sigma2[ip] += tv2sigma20;
        let t177 = t101 * t64;
        let t181 = t57 * t111;
        let t188 = t63 * t63;
        let t189 = 1.0 / t188;
        let t190 = t18 * t189;
        let t191 = t113 * t78;
        let t195 = t111 * t78;
        let t196 = t195 * t128;
        let t200 = 1.0 / t30 / t43;
        let t204 = t42 * t42;
        let t206 = 1.0 / t18 / t204;
        let t210 = -1540.0 / 6561.0 * t25 * t28 * t200 - 0.16304741809295789214e-1 * t39 * t41 * t206;
        let t215 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t32 * t52 + 0.11034063575830880331e0 * t62 * t177 * t78 + 0.33102190727492640993e0 * t62 * t181 * t113 - 0.16551095363746320496e0 * t62 * t106 * t128 - 0.99306572182477922976e0 * t62 * t190 * t191 + 0.99306572182477922976e0 * t139 * t196 - 0.16551095363746320496e0 * t62 * t65 * t210);
        let tv3rho30 = 2.0 * rho[ip] * t215 + 6.0 * t133;
        v3rho3[ip] += tv3rho30;
        let t222 = t62 * t57;
        let t228 = t189 * t93;
        let t229 = t228 * t113;
        let t232 = t111 * t150;
        let t233 = t232 * t78;
        let t236 = t140 * t128;
        let t245 = 110.0 / 2187.0 * t25 * t27 * t118 + 0.44467477661715788766e-2 * t39 * t89 * t124;
        let t250 = piecewise3::<f64>(t2, 0.0, 0.36780211919436267769e-1 * t62 * t177 * t93 + 0.22068127151661760662e0 * t222 * t141 - 0.11034063575830880331e0 * t62 * t106 * t150 - 0.99306572182477922976e0 * t139 * t229 + 0.66204381454985281984e0 * t139 * t233 + 0.33102190727492640992e0 * t139 * t236 - 0.16551095363746320496e0 * t62 * t65 * t245);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t250 + 4.0 * t155;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t256 = t189 * t158;
        let t257 = t256 * t78;
        let t260 = t140 * t150;
        let t263 = 1.0 / t72;
        let t268 = t62 * t162 * t111;
        let t270 = t39 * t26 * t78;
        let t274 = piecewise3::<f64>(t2, 0.0, 0.11034063575830880331e0 * t62 * t181 * t158 - 0.99306572182477922976e0 * t139 * t257 + 0.66204381454985281984e0 * t139 * t260 + 0.10894521661642854238e-3 * t62 * t263 * t166 + 0.4357808664657141695e-4 * t268 * t270);
        let tv3rhosigma20 = 2.0 * rho[ip] * t274 + 2.0 * t170;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t277 = t158 * t93;
        let t281 = t93 * t36;
        let t282 = t281 * t165;
        let t286 = piecewise3::<f64>(t2, 0.0, -0.99306572182477922976e0 * t62 * t190 * t277 + 0.13073425993971425085e-3 * t268 * t282);
        let tv3sigma30 = 2.0 * rho[ip] * t286;
        v3sigma3[ip] += tv3sigma30;
        let t293 = t32 * t64;
        let t297 = t101 * t111;
        let t304 = t57 * t189;
        let t314 = 1.0 / t188 / t49;
        let t315 = t18 * t314;
        let t316 = t113 * t113;
        let t324 = t128 * t128;
        let t346 = 10.0 / 27.0 * t6 * t17 * t68 * t52 - 0.24520141279624178513e0 * t62 * t293 * t78 - 0.44136254303323521324e0 * t62 * t297 * t113 + 0.22068127151661760662e0 * t62 * t177 * t128 - 0.13240876290997056397e1 * t62 * t304 * t191 + 0.13240876290997056397e1 * t222 * t196 - 0.22068127151661760661e0 * t62 * t106 * t210 + 0.3972262887299116919e1 * t62 * t315 * t316 - 0.59583943309486753786e1 * t139 * t189 * t113 * t128 + 0.99306572182477922976e0 * t62 * t112 * t324 + 0.13240876290997056397e1 * t139 * t195 * t210 - 0.16551095363746320496e0 * t62 * t65 * (26180.0 / 19683.0 * t25 * t28 / t30 / t72 + 0.13587284841079824345e0 * t39 * t41 / t18 / t204 / rho[ip]);
        let t347 = piecewise3::<f64>(t2, 0.0, t346);
        let tv4rho40 = 2.0 * rho[ip] * t347 + 8.0 * t215;
        v4rho4[ip] += tv4rho40;
        let t401 = 0.99306572182477922976e0 * t139 * t111 * t245 * t78 + 0.99306572182477922976e0 * t139 * t232 * t128 + 0.33102190727492640992e0 * t139 * t140 * t210 - 0.16551095363746320496e0 * t62 * t65 * (-1540.0 / 6561.0 * t25 * t27 * t200 - 0.32609483618591578428e-1 * t39 * t89 * t206) + 0.11034063575830880331e0 * t62 * t177 * t150 - 0.16551095363746320496e0 * t62 * t106 * t245 - 0.61300353199060446282e-1 * t62 * t293 * t93 - 0.22068127151661760662e0 * t62 * t101 * t141 + 0.66204381454985281985e0 * t222 * t233 + 0.33102190727492640993e0 * t222 * t236 - 0.29791971654743376893e1 * t139 * t189 * t150 * t113 - 0.99306572182477922978e0 * t222 * t229 + 0.3972262887299116919e1 * t139 * t314 * t93 * t191 - 0.29791971654743376893e1 * t139 * t228 * t78 * t128;
        let t402 = piecewise3::<f64>(t2, 0.0, t401);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t402 + 6.0 * t250;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t424 = t150 * t150;
        let t436 = t62 * t263 * t111;
        let t440 = t62 * t162 * t189;
        let t449 = -0.7356042383887253554e-1 * t62 * t297 * t158 - 0.66204381454985281985e0 * t222 * t257 + 0.44136254303323521323e0 * t222 * t260 + 0.3972262887299116919e1 * t139 * t314 * t158 * t113 - 0.3972262887299116919e1 * t139 * t228 * t78 * t150 - 0.99306572182477922976e0 * t139 * t256 * t128 + 0.66204381454985281984e0 * t62 * t112 * t424 + 0.66204381454985281984e0 * t139 * t140 * t245 - 0.65367129969857125428e-3 * t62 / t122 * t166 - 0.43578086646571416951e-3 * t436 * t270 - 0.13073425993971425085e-3 * t440 * t39 * t26 * t113 + 0.4357808664657141695e-4 * t268 * t39 * t26 * t128;
        let t450 = piecewise3::<f64>(t2, 0.0, t449);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t450 + 4.0 * t274;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t474 = piecewise3::<f64>(t2, 0.0, -0.33102190727492640992e0 * t62 * t304 * t277 + 0.3972262887299116919e1 * t139 * t314 * t277 * t78 - 0.29791971654743376893e1 * t139 * t256 * t150 - 0.65367129969857125425e-3 * t436 * t282 - 0.39220277981914275255e-3 * t440 * t281 * t165 * t78 + 0.13073425993971425085e-3 * t268 * t150 * t36 * t165);
        let tv4rhosigma30 = 2.0 * rho[ip] * t474 + 2.0 * t286;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t477 = t158 * t158;
        let t490 = t21 * t21;
        let t498 = piecewise3::<f64>(t2, 0.0, 0.3972262887299116919e1 * t62 * t315 * t477 - 0.78440555963828550511e-3 * t440 * t158 * t36 * t165 + 0.10326474681199677422e-6 * t62 / t18 / t204 / t29 * t111 * t20 / t23 / t490 * t27);
        let tv4sigma40 = 2.0 * rho[ip] * t498;
        v4sigma4[ip] += tv4sigma40;
    }
}
