//! GGA_X_VMT fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_vmt_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
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
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t38 = sigma[ip] * t29 * t33;
        let t41 = f64::exp(-param_alpha * t20 * t25 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t29 * t33 * t47;
        let t51 = 1.0 + t27 * t48 / 24.0;
        let t55 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t51);
        let tzk0 = 2.0 * t55;
        zk[ip] += tzk0;
        let t57 = t17 / t31;
        let t61 = t30 * rho[ip];
        let t63 = 1.0 / t31 / t61;
        let t65 = t29 * t63 * t47;
        let t68 = t20 * t20;
        let t69 = param_mu * t68;
        let t71 = 1.0 / t23 / t22;
        let t72 = sigma[ip] * sigma[ip];
        let t73 = t71 * t72;
        let t74 = t69 * t73;
        let t75 = t30 * t30;
        let t76 = t75 * t30;
        let t78 = 1.0 / t18 / t76;
        let t79 = t28 * t78;
        let t80 = param_alpha * t41;
        let t81 = t80 * t46;
        let t85 = param_mu * param_mu;
        let t86 = t85 * t68;
        let t87 = t86 * t73;
        let t88 = t45 * t45;
        let t89 = 1.0 / t88;
        let t90 = t41 * t89;
        let t91 = t79 * t90;
        let t94 = -t27 * t65 / 9.0 + t74 * t79 * t81 / 108.0 + t87 * t91 / 108.0;
        let t99 = piecewise3(t2, 0.0, -t6 * t57 * t51 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t55;
        vrho[ip] += tvrho0;
        let t104 = t71 * sigma[ip];
        let t106 = t75 * rho[ip];
        let t108 = 1.0 / t18 / t106;
        let t109 = t28 * t108;
        let t114 = t109 * t90;
        let t117 = t42 * t48 / 24.0 - t69 * t104 * t109 * t81 / 288.0 - t86 * t104 * t114 / 288.0;
        let t121 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let t126 = t17 / t31 / rho[ip];
        let t136 = t29 / t31 / t75 * t47;
        let t139 = t75 * t61;
        let t141 = 1.0 / t18 / t139;
        let t142 = t28 * t141;
        let t146 = t142 * t90;
        let t149 = t22 * t22;
        let t150 = 1.0 / t149;
        let t151 = param_mu * t150;
        let t152 = t72 * sigma[ip];
        let t153 = t151 * t152;
        let t154 = t75 * t75;
        let t155 = t154 * t30;
        let t156 = 1.0 / t155;
        let t157 = param_alpha * param_alpha;
        let t162 = t85 * t150;
        let t163 = t162 * t152;
        let t169 = t85 * param_mu * t150;
        let t170 = t169 * t152;
        let t173 = 1.0 / t88 / t45;
        let t177 = 11.0 / 27.0 * t27 * t136 - t74 * t142 * t81 / 12.0 - t87 * t146 / 12.0 + t153 * t156 * t157 * t47 / 81.0 + 2.0 / 81.0 * t163 * t156 * param_alpha * t90 + 2.0 / 81.0 * t170 * t156 * t41 * t173;
        let t182 = piecewise3(t2, 0.0, t6 * t126 * t51 / 12.0 - t6 * t57 * t94 / 4.0 - 3.0 / 8.0 * t6 * t19 * t177);
        let tv2rho20 = 2.0 * rho[ip] * t182 + 4.0 * t99;
        v2rho2[ip] += tv2rho20;
        let t190 = t71 * t28;
        let t191 = t69 * t190;
        let t192 = t78 * param_alpha;
        let t194 = sigma[ip] * t41 * t46;
        let t198 = t86 * t190;
        let t200 = t89 * sigma[ip];
        let t205 = t154 * rho[ip];
        let t206 = 1.0 / t205;
        let t218 = t206 * t41 * t173;
        let t221 = -t42 * t65 / 9.0 + t191 * t192 * t194 / 36.0 + t198 * t78 * t41 * t200 / 36.0 - t151 * t72 * t206 * t157 * t47 / 216.0 - t162 * t72 * t206 * param_alpha * t90 / 108.0 - t169 * t72 * t218 / 108.0;
        let t226 = piecewise3(t2, 0.0, -t6 * t57 * t117 / 8.0 - 3.0 / 8.0 * t6 * t19 * t221);
        let tv2rhosigma0 = 2.0 * rho[ip] * t226 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = t86 * t71;
        let t237 = 1.0 / t154;
        let t249 = t237 * t41 * t173;
        let t252 = -t191 * t108 * param_alpha * t47 / 144.0 - t233 * t114 / 144.0 + t151 * sigma[ip] * t237 * t157 * t47 / 576.0 + t162 * sigma[ip] * t237 * param_alpha * t90 / 288.0 + t169 * sigma[ip] * t249 / 288.0;
        let t256 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t252);
        let tv2sigma20 = 2.0 * rho[ip] * t256;
        v2sigma2[ip] += tv2sigma20;
    }
}
