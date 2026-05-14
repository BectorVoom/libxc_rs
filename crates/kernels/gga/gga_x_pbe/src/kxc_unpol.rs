//! GGA_X_PBE kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbe_kxc_unpol(
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
    param_kappa: f64,
    param_mu: f64,
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
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = param_kappa + param_mu * t20 * t25 * sigma[ip] * t28 * t33 / 24.0;
        let t42 = 1.0 + param_kappa * (1.0 - param_kappa / t37);
        let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        let t52 = t30 * rho[ip];
        let t56 = param_kappa * param_kappa;
        let t58 = t6 * t17 / t18 / t52 * t56;
        let t59 = t37 * t37;
        let t61 = 1.0 / t59 * param_mu;
        let t64 = t25 * sigma[ip] * t28;
        let t65 = t61 * t20 * t64;
        let t69 = piecewise3(t2, 0.0, -t6 * t17 / t31 * t42 / 8.0 + t58 * t65 / 24.0);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t78 = t20 * t25 * t28;
        let t79 = t61 * t78;
        let t82 = piecewise3(t2, 0.0, -t6 * t17 / t18 / t30 * t56 * t79 / 64.0);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
        let t91 = t30 * t30;
        let t96 = t6 * t17 / t18 / t91 * t56;
        let t99 = t91 * t52;
        let t103 = t6 * t17 / t99 * t56;
        let t106 = param_mu * param_mu;
        let t107 = 1.0 / t59 / t37 * t106;
        let t108 = t20 * t20;
        let t109 = t107 * t108;
        let t111 = 1.0 / t23 / t22;
        let t112 = sigma[ip] * sigma[ip];
        let t115 = t109 * t111 * t112 * t27;
        let t119 = piecewise3(t2, 0.0, t6 * t17 / t31 / rho[ip] * t42 / 12.0 - t96 * t65 / 8.0 + t103 * t115 / 54.0);
        let tv2rho20 = 2.0 * rho[ip] * t119 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t124 = t91 * t30;
        let t128 = t6 * t17 / t124 * t56;
        let t131 = t109 * t111 * t27 * sigma[ip];
        let t135 = piecewise3(t2, 0.0, 7.0 / 192.0 * t58 * t79 - t128 * t131 / 144.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t135 + 2.0 * t82;
        v2rhosigma[ip] += tv2rhosigma0;
        let t138 = t91 * rho[ip];
        let t145 = t107 * t108 * t111 * t27;
        let t148 = piecewise3(t2, 0.0, t6 * t17 / t138 * t56 * t145 / 384.0);
        let tv2sigma20 = 2.0 * rho[ip] * t148;
        v2sigma2[ip] += tv2sigma20;
        let t159 = t6 * t17 / t18 / t138 * t56;
        let t162 = t91 * t91;
        let t166 = t6 * t17 / t162 * t56;
        let t169 = t22 * t22;
        let t172 = t3 / t4 / t169;
        let t173 = t162 * t30;
        let t177 = t172 * t17 / t31 / t173;
        let t178 = t59 * t59;
        let t179 = 1.0 / t178;
        let t180 = t56 * t179;
        let t181 = t106 * param_mu;
        let t182 = t112 * sigma[ip];
        let t184 = t180 * t181 * t182;
        let t188 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t33 * t42 + 115.0 / 216.0 * t159 * t65 - 5.0 / 27.0 * t166 * t115 + 2.0 / 27.0 * t177 * t184);
        let tv3rho30 = 2.0 * rho[ip] * t188 + 6.0 * t119;
        v3rho3[ip] += tv3rho30;
        let t196 = t162 * rho[ip];
        let t200 = t172 * t17 / t31 / t196;
        let t202 = t180 * t181 * t112;
        let t206 = piecewise3(t2, 0.0, -35.0 / 288.0 * t96 * t79 + 25.0 / 432.0 * t103 * t131 - t200 * t202 / 36.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t206 + 4.0 * t135;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t212 = 1.0 / t31 / t162;
        let t216 = t180 * t181 * sigma[ip];
        let t220 = piecewise3(t2, 0.0, -5.0 / 384.0 * t128 * t145 + t172 * t17 * t212 * t216 / 96.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t220 + 2.0 * t148;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t223 = t172 * t17;
        let t227 = t179 * t181;
        let t231 = piecewise3(t2, 0.0, -t223 / t31 / t99 * t56 * t227 / 256.0);
        let tv3sigma30 = 2.0 * rho[ip] * t231;
        v3sigma3[ip] += tv3sigma30;
    }
}
