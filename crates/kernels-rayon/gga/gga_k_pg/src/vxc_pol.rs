//! GGA_K_PG vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pg_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_pg_mu: f64,
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = param_pg_mu * t32;
        let t47 = t36 * sigma0;
        let t51 = rmath::exp(-t46 * t47 * t42 / 24.0);
        let t52 = 5.0 / 72.0 * t37 * sigma0 * t42 + t51;
        let t56 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t17;
        let t60 = piecewise5(t15, t12, t11, t16, t58 * t8);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t24, t64 * t61);
        let t67 = t66 * t30;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t76 = t36 * sigma2;
        let t80 = rmath::exp(-t46 * t76 * t72 / 24.0);
        let t81 = 5.0 / 72.0 * t37 * sigma2 * t72 + t80;
        let t85 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t81);
        let tzk0 = t56 + t85;
        zk[ip] += tzk0;
        let t86 = t7 * t7;
        let t87 = 1.0 / t86;
        let t88 = t17 * t87;
        let t90 = piecewise5(t11, 0.0, t15, 0.0, t8 - t88);
        let t93 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t90);
        let t94 = t93 * t30;
        let t98 = 1.0 / t29;
        let t99 = t28 * t98;
        let t102 = t6 * t99 * t52 / 10.0;
        let t103 = t38 * rho0;
        let t105 = 1.0 / t40 / t103;
        let t106 = sigma0 * t105;
        let t109 = t46 * t36;
        let t113 = -5.0 / 27.0 * t37 * t106 + t109 * t106 * t51 / 9.0;
        let t118 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t94 * t52 + t102 + 3.0 / 20.0 * t6 * t31 * t113);
        let t119 = t58 * t87;
        let t121 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t119);
        let t124 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t121);
        let t125 = t124 * t30;
        let t129 = t66 * t98;
        let t132 = t6 * t129 * t81 / 10.0;
        let t134 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t125 * t81 + t132);
        let tvrho0 = t56 + t85 + t7 * (t118 + t134);
        vrho[ip * 2] += tvrho0;
        let t138 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t88);
        let t141 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t138);
        let t142 = t141 * t30;
        let t147 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t142 * t52 + t102);
        let t149 = piecewise5(t15, 0.0, t11, 0.0, t8 - t119);
        let t152 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t149);
        let t153 = t152 * t30;
        let t157 = t68 * rho1;
        let t159 = 1.0 / t70 / t157;
        let t160 = sigma2 * t159;
        let t166 = -5.0 / 27.0 * t37 * t160 + t109 * t160 * t80 / 9.0;
        let t171 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t153 * t81 + t132 + 3.0 / 20.0 * t6 * t67 * t166);
        let tvrho1 = t56 + t85 + t7 * (t147 + t171);
        vrho[ip * 2 + 1] += tvrho1;
        let t180 = 5.0 / 72.0 * t37 * t42 - t46 * t36 * t42 * t51 / 24.0;
        let t184 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t180);
        let tvsigma0 = t7 * t184;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t191 = 5.0 / 72.0 * t37 * t72 - t46 * t36 * t72 * t80 / 24.0;
        let t195 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t191);
        let tvsigma2 = t7 * t195;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
