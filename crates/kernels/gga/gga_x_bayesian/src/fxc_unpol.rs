//! GGA_X_BAYESIAN fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_bayesian_fxc_unpol(
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
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3::<f64>(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t20 * t20;
        let t35 = 1.0 / t22;
        let t36 = t34 * t35;
        let t37 = f64::sqrt(sigma[ip]);
        let t44 = 1.0 + t36 * t37 * t27 / t18 / rho[ip] / 12.0;
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t33 * t46;
        let t50 = 0.1926e0 + 0.79008333333333333333e-1 * t26 * t47;
        let t51 = t46 * t50;
        let t55 = 0.10008e1 + t26 * t33 * t51 / 24.0;
        let t59 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t55);
        let tzk0 = 2.0 * t59;
        zk[ip] += tzk0;
        let t61 = t17 / t30;
        let t65 = t29 * rho[ip];
        let t67 = 1.0 / t30 / t65;
        let t68 = t28 * t67;
        let t72 = 1.0 / t21;
        let t73 = t37 * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * rho[ip];
        let t77 = 1.0 / t76;
        let t79 = 1.0 / t45 / t44;
        let t80 = t77 * t79;
        let t84 = t68 * t46;
        let t89 = -0.21068888888888888889e0 * t26 * t84 + 0.21068888888888888889e0 * t74 * t80;
        let t90 = t46 * t89;
        let t94 = -t26 * t68 * t51 / 9.0 + t74 * t80 * t50 / 9.0 + t26 * t33 * t90 / 24.0;
        let t99 = piecewise3::<f64>(t2, 0.0, -t6 * t61 * t55 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t59;
        vrho[ip] += tvrho0;
        let t102 = t25 * t28;
        let t103 = t32 * t46;
        let t106 = t72 * t37;
        let t107 = 1.0 / t75;
        let t108 = t107 * t79;
        let t115 = 0.79008333333333333333e-1 * t25 * t47 - 0.79008333333333333333e-1 * t106 * t108;
        let t116 = t46 * t115;
        let t120 = t102 * t103 * t50 / 24.0 - t106 * t108 * t50 / 24.0 + t26 * t33 * t116 / 24.0;
        let t124 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t120);
        let tvsigma0 = 2.0 * rho[ip] * t124;
        vsigma[ip] += tvsigma0;
        let t129 = t17 / t30 / rho[ip];
        let t137 = 1.0 / t30 / t75;
        let t138 = t28 * t137;
        let t142 = t75 * t29;
        let t143 = 1.0 / t142;
        let t144 = t143 * t79;
        let t151 = sigma[ip] * sigma[ip];
        let t152 = t72 * t151;
        let t153 = t75 * t65;
        let t155 = 1.0 / t18 / t153;
        let t156 = t45 * t45;
        let t157 = 1.0 / t156;
        let t159 = t152 * t155 * t157;
        let t161 = t35 * t27;
        let t162 = t50 * t34 * t161;
        let t168 = t138 * t46;
        let t175 = t157 * t34 * t161;
        let t178 = 0.77252592592592592593e0 * t26 * t168 - 0.16152814814814814814e1 * t74 * t144 + 0.7022962962962962963e-1 * t152 * t155 * t175;
        let t179 = t46 * t178;
        let t183 = 11.0 / 27.0 * t26 * t138 * t51 - 23.0 / 27.0 * t74 * t144 * t50 - 2.0 / 9.0 * t26 * t68 * t90 + t159 * t162 / 27.0 + 2.0 / 9.0 * t74 * t80 * t89 + t26 * t33 * t179 / 24.0;
        let t188 = piecewise3::<f64>(t2, 0.0, t6 * t129 * t55 / 12.0 - t6 * t61 * t94 / 4.0 - 3.0 / 8.0 * t6 * t19 * t183);
        let tv2rho20 = 2.0 * rho[ip] * t188 + 4.0 * t99;
        v2rho2[ip] += tv2rho20;
        let t194 = t67 * t46;
        let t198 = t72 * t77;
        let t199 = t79 * t50;
        let t200 = t199 * t37;
        let t206 = t72 * sigma[ip];
        let t208 = 1.0 / t18 / t142;
        let t210 = t206 * t208 * t157;
        let t224 = t79 * t37;
        let t230 = -0.21068888888888888889e0 * t25 * t84 + 0.52672222222222222222e0 * t198 * t224 - 0.26336111111111111111e-1 * t206 * t208 * t175;
        let t231 = t46 * t230;
        let t235 = -t102 * t194 * t50 / 9.0 + 5.0 / 18.0 * t198 * t200 + t102 * t103 * t89 / 24.0 - t210 * t162 / 72.0 - t106 * t108 * t89 / 24.0 - t26 * t68 * t116 / 9.0 + t74 * t80 * t115 / 9.0 + t26 * t33 * t231 / 24.0;
        let t240 = piecewise3::<f64>(t2, 0.0, -t6 * t61 * t120 / 8.0 - 3.0 / 8.0 * t6 * t19 * t235);
        let tv2rhosigma0 = 2.0 * rho[ip] * t240 + 2.0 * t124;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = t72 * t107;
        let t244 = 1.0 / t37;
        let t245 = t199 * t244;
        let t253 = t72 / t18 / t76;
        let t254 = t253 * t157;
        let t260 = t79 * t244;
        let t263 = t36 * t27;
        let t266 = -0.1185125e0 * t243 * t260 + 0.98760416666666666666e-2 * t254 * t263;
        let t267 = t46 * t266;
        let t271 = -t243 * t245 / 16.0 + t102 * t103 * t115 / 12.0 + t254 * t162 / 192.0 - t106 * t108 * t115 / 12.0 + t26 * t33 * t267 / 24.0;
        let t275 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t271);
        let tv2sigma20 = 2.0 * rho[ip] * t275;
        v2sigma2[ip] += tv2sigma20;
    }
}
