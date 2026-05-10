//! GGA_X_LAG kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 25 shared lines across all orders.
//! Delta: 74 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lag_kxc_unpol(
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
        // --- shared preamble (25 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = t3 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = M_CBRT6;
        let t18 = t17 * t17;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = 1.0 / t20;
        let t22 = t18 * t21;
        let t23 = f64::sqrt(sigma[ip]);
        let t24 = M_CBRT2;
        let t29 = t22 * t23 * t24 / t16 / rho[ip];
        let t30 = f64::powf(t29, 0.2626712e1);
        let t33 = 1.0 + 0.13471619689594796103e-3 * t30;
        let t34 = f64::powf(t33, -0.657946e0);
        let t38 = piecewise3(t2, 0.0, -0.15400028771927569605e-4 * t15 * t16 * t30 * t34);
        let tzk0 = 2.0 * t38;
        zk[ip] += tzk0;
        // --- vxc delta (24 lines) ---
        let t39 = t16 * t16;
        let t45 = rho[ip] * rho[ip];
        let t46 = 1.0 / t45;
        let t47 = f64::powf(t29, 0.1626712e1);
        let t49 = t15 * t46 * t47;
        let t50 = t34 * t18;
        let t52 = t21 * t23 * t24;
        let t53 = t50 * t52;
        let t56 = f64::powf(t29, 0.4253424e1);
        let t58 = t15 * t46 * t56;
        let t59 = f64::powf(t33, -0.1657946e1);
        let t60 = t59 * t18;
        let t61 = t60 * t52;
        let t65 = piecewise3(t2, 0.0, -0.5133342923975856535e-5 * t15 / t39 * t30 * t34 + 0.53935253834089880284e-4 * t49 * t53 - 0.47806042356233315032e-8 * t58 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t38;
        vrho[ip] += tvrho0;
        let t68 = 1.0 / rho[ip];
        let t70 = t15 * t68 * t47;
        let t71 = 1.0 / t23;
        let t73 = t21 * t71 * t24;
        let t74 = t50 * t73;
        let t78 = t15 * t68 * t56;
        let t79 = t60 * t73;
        let t83 = piecewise3(t2, 0.0, -0.20225720187783705106e-4 * t70 * t74 + 0.17927265883587493137e-8 * t78 * t79);
        let tvsigma0 = 2.0 * rho[ip] * t83;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (51 lines) ---
        let t92 = t45 * rho[ip];
        let t93 = 1.0 / t92;
        let t95 = t15 * t93 * t47;
        let t99 = t15 * t93 * t56;
        let t102 = t45 * t45;
        let t104 = 1.0 / t16 / t102;
        let t105 = f64::powf(t29, 0.626712e0);
        let t107 = t15 * t104 * t105;
        let t108 = t34 * t17;
        let t109 = t20 * t20;
        let t110 = 1.0 / t109;
        let t112 = t24 * t24;
        let t113 = t110 * sigma[ip] * t112;
        let t114 = t108 * t113;
        let t117 = f64::powf(t29, 0.3253424e1);
        let t119 = t15 * t104 * t117;
        let t120 = t59 * t17;
        let t121 = t120 * t113;
        let t124 = f64::powf(t29, 0.5880136e1);
        let t126 = t15 * t104 * t124;
        let t127 = f64::powf(t33, -0.2657946e1);
        let t128 = t127 * t17;
        let t129 = t128 * t113;
        let t133 = piecewise3(t2, 0.0, 0.34222286159839043567e-5 * t15 / t39 / rho[ip] * t30 * t34 - 0.89892089723483133809e-4 * t95 * t53 + 0.79676737260388858387e-8 * t99 * t61 - 0.70189699707968013869e-3 * t107 * t114 + 0.26312965842611652413e-6 * t119 * t121 - 0.22437549929142988057e-10 * t126 * t129);
        let tv2rho20 = 2.0 * rho[ip] * t133 + 4.0 * t65;
        v2rho2[ip] += tv2rho20;
        let t139 = 1.0 / t16 / t92;
        let t141 = t15 * t139 * t105;
        let t142 = t110 * t112;
        let t143 = t108 * t142;
        let t147 = t15 * t139 * t117;
        let t148 = t120 * t142;
        let t154 = t15 * t139 * t124;
        let t155 = t128 * t142;
        let t159 = piecewise3(t2, 0.0, 0.20225720187783705106e-4 * t49 * t74 + 0.263211373904880052e-3 * t141 * t143 - 0.98673621909793696545e-7 * t147 * t148 - 0.17927265883587493137e-8 * t58 * t79 + 0.84140812234286205216e-11 * t154 * t155);
        let tv2rhosigma0 = 2.0 * rho[ip] * t159 + 2.0 * t83;
        v2rhosigma[ip] += tv2rhosigma0;
        let t163 = 1.0 / t16 / t45;
        let t165 = t15 * t163 * t105;
        let t166 = 1.0 / sigma[ip];
        let t168 = t110 * t166 * t112;
        let t169 = t108 * t168;
        let t173 = t15 * t163 * t117;
        let t174 = t120 * t168;
        let t177 = t23 * sigma[ip];
        let t178 = 1.0 / t177;
        let t180 = t21 * t178 * t24;
        let t181 = t50 * t180;
        let t185 = t15 * t163 * t124;
        let t186 = t128 * t168;
        let t189 = t60 * t180;
        let t193 = piecewise3(t2, 0.0, -0.98704265214330019501e-4 * t165 * t169 + 0.37002608216172636205e-7 * t173 * t174 + 0.10112860093891852553e-4 * t70 * t181 - 0.31552804587857326957e-11 * t185 * t186 - 0.89636329417937465685e-9 * t78 * t189);
        let tv2sigma20 = 2.0 * rho[ip] * t193;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (74 lines) ---
        let t202 = 1.0 / t102;
        let t204 = t15 * t202 * t47;
        let t208 = t15 * t202 * t56;
        let t211 = t102 * rho[ip];
        let t213 = 1.0 / t16 / t211;
        let t215 = t15 * t213 * t105;
        let t219 = t15 * t213 * t117;
        let t223 = t15 * t213 * t124;
        let t226 = t102 * t45;
        let t229 = t15 / t39 / t226;
        let t230 = f64::powf(t29, -0.373288e0);
        let t231 = t230 * t34;
        let t232 = 1.0 / t19;
        let t233 = t232 * t177;
        let t234 = t231 * t233;
        let t237 = f64::powf(t29, 0.2253424e1);
        let t238 = t237 * t59;
        let t239 = t238 * t177;
        let t242 = t238 * t233;
        let t245 = f64::powf(t29, 0.4880136e1);
        let t246 = t245 * t127;
        let t247 = t246 * t177;
        let t250 = t246 * t233;
        let t253 = f64::powf(t29, 0.7506848e1);
        let t254 = f64::powf(t33, -0.3657946e1);
        let t255 = t253 * t254;
        let t256 = t255 * t177;
        let t259 = -0.57037143599731739278e-5 * t15 / t39 / t45 * t30 * t34 + 0.25769065720731831692e-3 * t204 * t53 - 0.22840664681311472738e-7 * t208 * t61 + 0.42113819824780808322e-2 * t215 * t114 - 0.15787779505566991448e-5 * t219 * t121 + 0.13462529957485792834e-9 * t223 * t129 + 0.70381963333408079853e-2 * t229 * t234 - 0.2649208530018449667e-6 * t229 * t239 - 0.13697157533365275622e-4 * t229 * t242 + 0.25026072169985681153e-9 * t229 * t247 + 0.21109735214424181315e-8 * t229 * t250 - 0.34211655888476305258e-13 * t229 * t256;
        let t260 = piecewise3(t2, 0.0, t259);
        let tv3rho30 = 2.0 * rho[ip] * t260 + 6.0 * t133;
        v3rho3[ip] += tv3rho30;
        let t272 = t15 / t39 / t211;
        let t273 = t232 * t23;
        let t274 = t231 * t273;
        let t277 = t238 * t23;
        let t280 = t238 * t273;
        let t283 = t246 * t23;
        let t290 = t246 * t273;
        let t293 = t255 * t23;
        let t296 = -0.40451440375567410212e-4 * t95 * t74 - 0.11405826202544802253e-2 * t107 * t143 + 0.4275856949424393517e-6 * t119 * t148 - 0.26393236250028029944e-2 * t272 * t274 + 0.9934531987569186251e-7 * t272 * t277 + 0.51364340750119783582e-5 * t272 * t280 - 0.93847770637446304319e-10 * t272 * t283 + 0.35854531767174986274e-8 * t99 * t79 - 0.36461018634857355594e-10 * t126 * t155 - 0.79161507054090679935e-9 * t272 * t290 + 0.12829370958178614472e-13 * t272 * t293;
        let t297 = piecewise3(t2, 0.0, t296);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t297 + 4.0 * t159;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t304 = t15 / t39 / t102;
        let t305 = t232 * t71;
        let t306 = t231 * t305;
        let t309 = t238 * t71;
        let t314 = t238 * t305;
        let t317 = t246 * t71;
        let t324 = t246 * t305;
        let t327 = t255 * t71;
        let t332 = 0.987042652143300195e-4 * t141 * t169 + 0.9897463593760511229e-3 * t304 * t306 - 0.37254494953384448442e-7 * t304 * t309 - 0.37002608216172636205e-7 * t147 * t174 - 0.19261627781294918844e-5 * t304 * t314 + 0.3519291398904236412e-10 * t304 * t317 - 0.10112860093891852553e-4 * t49 * t181 + 0.31552804587857326958e-11 * t154 * t186 + 0.29685565145284004977e-9 * t304 * t324 - 0.48110141093169804271e-14 * t304 * t327 + 0.89636329417937465685e-9 * t58 * t189;
        let t333 = piecewise3(t2, 0.0, t332);
        let tv3rhosigma20 = 2.0 * rho[ip] * t333 + 2.0 * t193;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t337 = 1.0 / t39 / t92;
        let t338 = t15 * t337;
        let t339 = t232 * t178;
        let t340 = t231 * t339;
        let t343 = t238 * t178;
        let t346 = sigma[ip] * sigma[ip];
        let t347 = 1.0 / t346;
        let t349 = t110 * t347 * t112;
        let t350 = t108 * t349;
        let t353 = t238 * t339;
        let t356 = t246 * t178;
        let t359 = t120 * t349;
        let t363 = 1.0 / t23 / t346;
        let t365 = t21 * t363 * t24;
        let t366 = t50 * t365;
        let t369 = t246 * t339;
        let t372 = t255 * t178;
        let t375 = t128 * t349;
        let t378 = t60 * t365;
        let t381 = -0.37115488476601917109e-3 * t338 * t340 + 0.13970435607519168166e-7 * t338 * t343 + 0.14805639782149502925e-3 * t165 * t350 + 0.72231104179855945664e-6 * t338 * t353 - 0.13197342745890886545e-10 * t338 * t356 - 0.55503912324258954307e-7 * t173 * t359 - 0.1516929014083777883e-4 * t70 * t366 - 0.11132086929481501866e-9 * t338 * t369 + 0.18041302909938676602e-14 * t338 * t372 + 0.47329206881785990436e-11 * t185 * t375 + 0.13445449412690619853e-8 * t78 * t378;
        let t382 = piecewise3(t2, 0.0, t381);
        let tv3sigma30 = 2.0 * rho[ip] * t382;
        v3sigma3[ip] += tv3sigma30;
    }
}
