//! GGA_X_RPBE lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_rpbe_lxc_unpol(
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
    param_rpbe_kappa: f64,
    param_rpbe_mu: f64,
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
        let t21 = param_rpbe_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = 1.0 / param_rpbe_kappa;
        let t39 = f64::exp(-t21 * t25 * t29 * t33 * t34 / 24.0);
        let t42 = 1.0 + param_rpbe_kappa * (1.0 - t39);
        let t46 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        let t52 = t30 * rho[ip];
        let t55 = t17 / t18 / t52;
        let t59 = t29 * t39;
        let t60 = t20 * t25 * t59;
        let t64 = piecewise3::<f64>(t2, 0.0, -t6 * t17 / t31 * t42 / 8.0 + t6 * t55 * param_rpbe_mu * t60 / 24.0);
        let tvrho0 = 2.0 * rho[ip] * t64 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t72 = t25 * t28 * t39;
        let t73 = t21 * t72;
        let t76 = piecewise3::<f64>(t2, 0.0, -t6 * t17 / t18 / t30 * t73 / 64.0);
        let tvsigma0 = 2.0 * rho[ip] * t76;
        vsigma[ip] += tvsigma0;
        let t85 = t30 * t30;
        let t88 = t17 / t18 / t85;
        let t93 = t85 * t52;
        let t96 = param_rpbe_mu * param_rpbe_mu;
        let t98 = t6 * t17 / t93 * t96;
        let t99 = t20 * t20;
        let t102 = t99 / t23 / t22;
        let t103 = sigma[ip] * sigma[ip];
        let t106 = t27 * t34 * t39;
        let t107 = t102 * t103 * t106;
        let t111 = piecewise3::<f64>(t2, 0.0, t6 * t17 / t31 / rho[ip] * t42 / 12.0 - t6 * t88 * param_rpbe_mu * t60 / 8.0 + t98 * t107 / 108.0);
        let tv2rho20 = 2.0 * rho[ip] * t111 + 4.0 * t64;
        v2rho2[ip] += tv2rho20;
        let t117 = t85 * t30;
        let t121 = t6 * t17 / t117 * t96;
        let t125 = t102 * t27 * sigma[ip] * t34 * t39;
        let t129 = piecewise3::<f64>(t2, 0.0, 7.0 / 192.0 * t6 * t55 * t73 - t121 * t125 / 288.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t129 + 2.0 * t76;
        v2rhosigma[ip] += tv2rhosigma0;
        let t132 = t85 * rho[ip];
        let t137 = t102 * t106;
        let t140 = piecewise3::<f64>(t2, 0.0, t6 * t17 / t132 * t96 * t137 / 768.0);
        let tv2sigma20 = 2.0 * rho[ip] * t140;
        v2sigma2[ip] += tv2sigma20;
        let t149 = t17 / t18 / t132;
        let t154 = t85 * t85;
        let t158 = t6 * t17 / t154 * t96;
        let t161 = t22 * t22;
        let t164 = t3 / t4 / t161;
        let t165 = t154 * t30;
        let t169 = t164 * t17 / t31 / t165;
        let t170 = t96 * param_rpbe_mu;
        let t171 = t103 * sigma[ip];
        let t173 = param_rpbe_kappa * param_rpbe_kappa;
        let t174 = 1.0 / t173;
        let t175 = t174 * t39;
        let t176 = t170 * t171 * t175;
        let t180 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t33 * t42 + 115.0 / 216.0 * t6 * t149 * param_rpbe_mu * t60 - 5.0 / 54.0 * t158 * t107 + t169 * t176 / 81.0);
        let tv3rho30 = 2.0 * rho[ip] * t180 + 6.0 * t111;
        v3rho3[ip] += tv3rho30;
        let t189 = t154 * rho[ip];
        let t193 = t164 * t17 / t31 / t189;
        let t195 = t170 * t103 * t175;
        let t199 = piecewise3::<f64>(t2, 0.0, -35.0 / 288.0 * t6 * t88 * t73 + 25.0 / 864.0 * t98 * t125 - t193 * t195 / 216.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t199 + 4.0 * t129;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t205 = 1.0 / t31 / t154;
        let t210 = t170 * t174 * sigma[ip] * t39;
        let t214 = piecewise3::<f64>(t2, 0.0, -5.0 / 768.0 * t121 * t137 + t164 * t17 * t205 * t210 / 576.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t214 + 2.0 * t140;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t217 = t164 * t17;
        let t224 = piecewise3::<f64>(t2, 0.0, -t217 / t31 / t93 * t170 * t175 / 1536.0);
        let tv3sigma30 = 2.0 * rho[ip] * t224;
        v3sigma3[ip] += tv3sigma30;
        let t246 = t154 * t52;
        let t257 = t96 * t96;
        let t260 = t103 * t103;
        let t262 = 1.0 / t173 / param_rpbe_kappa;
        let t269 = piecewise3::<f64>(t2, 0.0, 10.0 / 27.0 * t6 * t17 / t31 / t52 * t42 - 305.0 / 108.0 * t6 * t17 / t18 / t117 * param_rpbe_mu * t60 + 835.0 / 972.0 * t6 * t17 / t189 * t96 * t107 - 62.0 / 243.0 * t164 * t17 / t31 / t246 * t176 + t164 * t17 / t18 / t154 / t117 * t257 * t260 * t262 * t20 * t72 / 729.0);
        let tv4rho40 = 2.0 * rho[ip] * t269 + 8.0 * t180;
        v4rho4[ip] += tv4rho40;
        let t292 = piecewise3::<f64>(t2, 0.0, 455.0 / 864.0 * t6 * t149 * t73 - 595.0 / 2592.0 * t158 * t125 + t169 * t195 / 12.0 - t164 * t17 / t18 / t154 / t132 * t257 * t171 * t262 * t20 * t72 / 1944.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t292 + 6.0 * t199;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t312 = piecewise3::<f64>(t2, 0.0, 5.0 / 128.0 * t98 * t137 - 41.0 / 1728.0 * t193 * t210 + t164 * t17 / t18 / t154 / t85 * t257 * t262 * t103 * t20 * t72 / 5184.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t312 + 4.0 * t214;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t324 = t262 * t20;
        let t330 = piecewise3::<f64>(t2, 0.0, 23.0 / 4608.0 * t217 * t205 * t170 * t175 - t164 * t17 / t18 / t246 * t257 * t324 * t25 * t59 / 13824.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t330 + 2.0 * t224;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t341 = piecewise3::<f64>(t2, 0.0, t164 * t17 / t18 / t165 * t257 * t324 * t72 / 36864.0);
        let tv4sigma40 = 2.0 * rho[ip] * t341;
        v4sigma4[ip] += tv4sigma40;
    }
}
