//! HYB_MGGA_X_DLDF fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_mgga_x_dldf_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = pow_1_3(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = t4 * t15;
        let t17 = pow_1_3(rho[ip]);
        let t18 = M_CBRT6;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = t20 * t20;
        let t23 = t18 / t21;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = t17 * t17;
        let t30 = 1.0 / t28 / t27;
        let t34 = 0.48827323e1 + 0.146297e-1 * t23 * t26 * t30;
        let t37 = 0.58827323e1 - 0.2384107471346329e2 / t34;
        let t38 = t17 * t37;
        let t39 = t18 * t18;
        let t41 = 3.0 / 10.0 * t39 * t21;
        let t42 = tau[ip] * t25;
        let t44 = 1.0 / t28 / rho[ip];
        let t45 = t42 * t44;
        let t46 = t41 - t45;
        let t47 = t41 + t45;
        let t48 = 1.0 / t47;
        let t51 = t46 * t46;
        let t52 = t47 * t47;
        let t53 = 1.0 / t52;
        let t56 = t51 * t46;
        let t57 = t52 * t47;
        let t58 = 1.0 / t57;
        let t61 = t51 * t51;
        let t62 = t52 * t52;
        let t63 = 1.0 / t62;
        let t66 = 1.0 - 0.1637571e0 * t46 * t48 - 0.1880028e0 * t51 * t53 - 0.4490609e0 * t56 * t58 - 0.82359e-2 * t61 * t63;
        let t70 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = 1.0 / t28 * t37;
        let t76 = t27 * rho[ip];
        let t78 = 1.0 / t17 / t76;
        let t79 = t34 * t34;
        let t80 = 1.0 / t79;
        let t82 = t16 * t78 * t80;
        let t84 = t23 * t26 * t66;
        let t90 = t46 * t53;
        let t91 = t42 * t30;
        let t94 = t51 * t58;
        let t97 = t56 * t63;
        let t101 = 1.0 / t62 / t47;
        let t102 = t61 * t101;
        let t105 = -0.2729285e0 * t42 * t30 * t48 - 0.8996045e0 * t90 * t91 - 0.28719805e1 * t94 * t91 - 0.23002105e1 * t97 * t91 - 0.54906e-1 * t102 * t91;
        let t110 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t66 + 0.91826307502838492063e-1 * t82 * t84 - 0.98727272578809758046e-1 * t16 * t38 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t116 = t16 / t17 / t27 * t80;
        let t118 = t23 * t25 * t66;
        let t121 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t118);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t123 = t25 * t44;
        let t134 = 0.1637571e0 * t123 * t48 + 0.5397627e0 * t90 * t123 + 0.17231883e1 * t94 * t123 + 0.13801263e1 * t97 * t123 + 0.329436e-1 * t102 * t123;
        let t138 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t134);
        let tvtau0 = 2.0 * rho[ip] * t138;
        vtau[ip] += tvtau0;
        let t141 = t44 * t37;
        let t145 = t27 * t27;
        let t147 = 1.0 / t17 / t145;
        let t149 = t16 * t147 * t80;
        let t155 = t145 * t76;
        let t156 = 1.0 / t155;
        let t158 = 1.0 / t79 / t34;
        let t160 = t16 * t156 * t158;
        let t163 = t39 / t20 / t19;
        let t164 = sigma[ip] * sigma[ip];
        let t165 = t164 * t24;
        let t167 = t163 * t165 * t66;
        let t171 = t23 * t26 * t105;
        let t175 = 1.0 / t28 / t76;
        let t179 = tau[ip] * tau[ip];
        let t180 = t179 * t24;
        let t181 = t145 * rho[ip];
        let t183 = 1.0 / t17 / t181;
        let t187 = t46 * t58;
        let t188 = t180 * t183;
        let t191 = t42 * t175;
        let t194 = t51 * t63;
        let t199 = t56 * t101;
        let t205 = 1.0 / t62 / t52;
        let t206 = t61 * t205;
        let t211 = 0.72780933333333333333e0 * t42 * t175 * t48 - 0.39084433333333333334e1 * t180 * t183 * t53 - 0.251439e2 * t187 * t188 + 0.23989453333333333333e1 * t90 * t191 - 0.5172191e2 * t194 * t188 + 0.76586146666666666667e1 * t94 * t191 - 0.31401553333333333333e2 * t199 * t188 + 0.61338946666666666667e1 * t97 * t191 - 0.9151e0 * t206 * t188 + 0.146416e0 * t102 * t191;
        let t216 = piecewise3(t3, 0.0, 0.21939393906402168455e-1 * t16 * t141 * t66 - 0.27547892250851547619e0 * t149 * t84 - 0.65818181719206505364e-1 * t16 * t72 * t105 + 0.14329507529325613731e-1 * t160 * t167 + 0.18365261500567698413e0 * t82 * t171 - 0.98727272578809758046e-1 * t16 * t38 * t211);
        let tv2rho20 = 2.0 * rho[ip] * t216 + 4.0 * t110;
        v2rho2[ip] += tv2rho20;
        let t221 = t145 * t27;
        let t222 = 1.0 / t221;
        let t224 = t16 * t222 * t158;
        let t225 = t24 * t66;
        let t227 = t163 * t225 * sigma[ip];
        let t231 = t23 * t25 * t105;
        let t235 = piecewise3(t3, 0.0, 0.80348019064983680556e-1 * t82 * t118 - 0.53735653234971051493e-2 * t224 * t227 - 0.34434865313564434524e-1 * t116 * t231);
        let tv2rhosigma0 = 2.0 * rho[ip] * t235 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t242 = t23 * t26 * t134;
        let t245 = t25 * t30;
        let t248 = t24 * t147;
        let t249 = t53 * tau[ip];
        let t252 = t248 * tau[ip];
        let t269 = -0.2729285e0 * t245 * t48 + 0.2345066e1 * t248 * t249 + 0.1508634e2 * t187 * t252 - 0.8996045e0 * t90 * t245 + 0.31033146e2 * t194 * t252 - 0.28719805e1 * t94 * t245 + 0.18840932e2 * t199 * t252 - 0.23002105e1 * t97 * t245 + 0.54906e0 * t206 * t252 - 0.54906e-1 * t102 * t245;
        let t274 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t134 + 0.91826307502838492063e-1 * t82 * t242 - 0.98727272578809758046e-1 * t16 * t38 * t269);
        let tv2rhotau0 = 2.0 * rho[ip] * t274 + 2.0 * t138;
        v2rhotau[ip] += tv2rhotau0;
        let t277 = 1.0 / t181;
        let t279 = t16 * t277 * t158;
        let t280 = t163 * t225;
        let t283 = piecewise3(t3, 0.0, 0.2015086996311414431e-2 * t279 * t280);
        let tv2sigma20 = 2.0 * rho[ip] * t283;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t286 = t23 * t25 * t134;
        let t289 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t286);
        let tv2sigmatau0 = 2.0 * rho[ip] * t289;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t291 = t24 * t78;
        let t302 = -0.14070396e1 * t291 * t53 - 0.9051804e1 * t187 * t291 - 0.186198876e2 * t194 * t291 - 0.113045592e2 * t199 * t291 - 0.329436e0 * t206 * t291;
        let t306 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t302);
        let tv2tau20 = 2.0 * rho[ip] * t306;
        v2tau2[ip] += tv2tau20;
    }
}
