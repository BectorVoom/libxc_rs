//! GGA_X_PBEA vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbea_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t28 = rho0 * rho0;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / t28;
        let t35 = 1.0 + 0.008639940809536326 * sigma0 * t32;
        let t36 = rmath::pow(t35, -0.52);
        let t38 = 1.804 - 0.804 * t36;
        let t42 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t38);
        let t43 = rho1 <= dens_threshold;
        let t44 = -t16;
        let t46 = piecewise5(t14, t11, t10, t15, t44 * t7);
        let t47 = 1.0 + t46;
        let t48 = t47 <= zeta_threshold;
        let t49 = pow_1_3(t47);
        let t51 = piecewise3(t48, t22, t49 * t47);
        let t53 = rho1 * rho1;
        let t54 = pow_1_3(rho1);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / t53;
        let t60 = 1.0 + 0.008639940809536326 * sigma2 * t57;
        let t61 = rmath::pow(t60, -0.52);
        let t63 = 1.804 - 0.804 * t61;
        let t67 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t51 * t26 * t63);
        let tzk0 = t42 + t67;
        zk[ip] += tzk0;
        let t68 = t6 * t6;
        let t69 = 1.0 / t68;
        let t70 = t16 * t69;
        let t72 = piecewise5(t10, 0.0, t14, 0.0, t7 - t70);
        let t75 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t72);
        let t80 = t26 * t26;
        let t81 = 1.0 / t80;
        let t85 = t5 * t25 * t81 * t38 / 8.0;
        let t86 = t2 * t25;
        let t87 = t86 * t26;
        let t88 = rmath::pow(t35, -1.52);
        let t89 = t88 * sigma0;
        let t90 = t28 * rho0;
        let t92 = 1.0 / t30 / t90;
        let t93 = t89 * t92;
        let t97 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t75 * t26 * t38 - t85 + 0.00246634334405953 * t87 * t93);
        let t98 = t44 * t69;
        let t100 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t98);
        let t103 = piecewise3(t48, 0.0, 4.0 / 3.0 * t49 * t100);
        let t111 = t5 * t51 * t81 * t63 / 8.0;
        let t113 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t103 * t26 * t63 - t111);
        let tvrho0 = t42 + t67 + t6 * (t97 + t113);
        vrho[ip * 2] += tvrho0;
        let t117 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t70);
        let t120 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t117);
        let t126 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t120 * t26 * t38 - t85);
        let t128 = piecewise5(t14, 0.0, t10, 0.0, t7 - t98);
        let t131 = piecewise3(t48, 0.0, 4.0 / 3.0 * t49 * t128);
        let t136 = t2 * t51;
        let t137 = t136 * t26;
        let t138 = rmath::pow(t60, -1.52);
        let t139 = t138 * sigma2;
        let t140 = t53 * rho1;
        let t142 = 1.0 / t55 / t140;
        let t143 = t139 * t142;
        let t147 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t131 * t26 * t63 - t111 + 0.00246634334405953 * t137 * t143);
        let tvrho1 = t42 + t67 + t6 * (t126 + t147);
        vrho[ip * 2 + 1] += tvrho1;
        let t150 = t26 * t88;
        let t151 = t150 * t32;
        let t154 = piecewise3(t1, 0.0, -0.0009248787540223239 * t86 * t151);
        let tvsigma0 = t6 * t154;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t155 = t26 * t138;
        let t156 = t155 * t57;
        let t159 = piecewise3(t43, 0.0, -0.0009248787540223239 * t136 * t156);
        let tvsigma2 = t6 * t159;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
