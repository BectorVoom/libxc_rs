//! GGA_X_AIRY vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_airy_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = rmath::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t32 = t25 * t28 * t30;
        let t33 = rmath::pow(t32, 2.626712);
        let t35 = 1.0 + 0.00013471619689594795 * t33;
        let t36 = rmath::pow(t35, -0.657946);
        let t39 = rmath::pow(t32, 3.217063);
        let t41 = rmath::pow(t32, 3.223476);
        let t43 = 1.0 - 0.04521241301076986 * t39 + 0.04540222195662038 * t41;
        let t44 = rmath::pow(t32, 3.473804);
        let t46 = 1.0 + 0.0004770218022490335 * t44;
        let t47 = 1.0 / t46;
        let t49 = 6.014601922021111e-05 * t33 * t36 + t43 * t47;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t54 = t18 * t18;
        let t56 = t17 / t54;
        let t60 = rmath::pow(t32, 1.626712);
        let t62 = t60 * t36 * t21;
        let t63 = t24 * t26;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t18 / t64;
        let t67 = t27 * t66;
        let t68 = t63 * t67;
        let t71 = rmath::pow(t32, 4.253424);
        let t72 = rmath::pow(t35, -1.657946);
        let t74 = t71 * t72 * t21;
        let t77 = rmath::pow(t32, 2.217063);
        let t79 = t77 * t21 * t24;
        let t80 = t28 * t66;
        let t83 = rmath::pow(t32, 2.223476);
        let t85 = t83 * t21 * t24;
        let t88 = 0.19393490805022173 * t79 * t80 - 0.19513729709845176 * t85 * t80;
        let t90 = t46 * t46;
        let t91 = 1.0 / t90;
        let t92 = t43 * t91;
        let t93 = rmath::pow(t32, 2.473804);
        let t94 = t93 * t21;
        let t95 = t92 * t94;
        let t98 = -0.00021064836058394556 * t62 * t68 + 1.8671024483029836e-08 * t74 * t68 + t88 * t47 + 0.0022094403263198687 * t95 * t68;
        let t103 = piecewise3(t2, 0.0, -t6 * t56 * t49 / 8.0 - 3.0 / 8.0 * t6 * t19 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t106 = 1.0 / t26;
        let t107 = t24 * t106;
        let t108 = t27 * t30;
        let t109 = t107 * t108;
        let t114 = t106 * t27;
        let t115 = t114 * t30;
        let t120 = -0.07272559051883315 * t79 * t115 + 0.07317648641191941 * t85 * t115;
        let t124 = 7.899313521897959e-05 * t62 * t109 - 7.001634181136188e-09 * t74 * t109 + t120 * t47 - 0.0008285401223699508 * t95 * t109;
        let t128 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
    }
}
