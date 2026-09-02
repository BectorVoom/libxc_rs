//! GGA_X_VMT vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_vmt_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_alpha: f64,
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
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = param_alpha * t28;
        let t42 = t33 * sigma0;
        let t43 = t42 * t39;
        let t46 = rmath::exp(-t41 * t43 / 24.0);
        let t49 = 1.0 + t29 * t43 / 24.0;
        let t50 = 1.0 / t49;
        let t51 = t46 * t50;
        let t55 = 1.0 + t34 * sigma0 * t39 * t51 / 24.0;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = rho1 * rho1;
        let t71 = pow_1_3(rho1);
        let t72 = t71 * t71;
        let t74 = 1.0 / t72 / t70;
        let t76 = t33 * sigma2;
        let t77 = t76 * t74;
        let t80 = rmath::exp(-t41 * t77 / 24.0);
        let t83 = 1.0 + t29 * t77 / 24.0;
        let t84 = 1.0 / t83;
        let t85 = t80 * t84;
        let t89 = 1.0 + t34 * sigma2 * t74 * t85 / 24.0;
        let t93 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t89);
        let tzk0 = t59 + t93;
        zk[ip] += tzk0;
        let t94 = t6 * t6;
        let t95 = 1.0 / t94;
        let t96 = t16 * t95;
        let t98 = piecewise5(t10, 0.0, t14, 0.0, t7 - t96);
        let t101 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t98);
        let t102 = t101 * t26;
        let t106 = t26 * t26;
        let t107 = 1.0 / t106;
        let t108 = t25 * t107;
        let t111 = t5 * t108 * t55 / 8.0;
        let t112 = t35 * rho0;
        let t114 = 1.0 / t37 / t112;
        let t119 = t28 * t28;
        let t120 = param_mu * t119;
        let t122 = 1.0 / t31 / t30;
        let t123 = sigma0 * sigma0;
        let t125 = t120 * t122 * t123;
        let t126 = t35 * t35;
        let t127 = t126 * t35;
        let t129 = 1.0 / t36 / t127;
        let t131 = t129 * param_alpha * t51;
        let t134 = param_mu * param_mu;
        let t136 = t134 * t119 * t122;
        let t138 = t49 * t49;
        let t139 = 1.0 / t138;
        let t140 = t46 * t139;
        let t144 = -t34 * sigma0 * t114 * t51 / 9.0 + t125 * t131 / 216.0 + t136 * t123 * t129 * t140 / 216.0;
        let t149 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t102 * t55 - t111 - 3.0 / 8.0 * t5 * t27 * t144);
        let t150 = t61 * t95;
        let t152 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t150);
        let t155 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t152);
        let t156 = t155 * t26;
        let t160 = t68 * t107;
        let t163 = t5 * t160 * t89 / 8.0;
        let t165 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t156 * t89 - t163);
        let tvrho0 = t59 + t93 + t6 * (t149 + t165);
        vrho[ip * 2] += tvrho0;
        let t169 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t96);
        let t172 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t169);
        let t173 = t172 * t26;
        let t178 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t173 * t55 - t111);
        let t180 = piecewise5(t14, 0.0, t10, 0.0, t7 - t150);
        let t183 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t180);
        let t184 = t183 * t26;
        let t188 = t70 * rho1;
        let t190 = 1.0 / t72 / t188;
        let t195 = sigma2 * sigma2;
        let t197 = t120 * t122 * t195;
        let t198 = t70 * t70;
        let t199 = t198 * t70;
        let t201 = 1.0 / t71 / t199;
        let t203 = t201 * param_alpha * t85;
        let t207 = t83 * t83;
        let t208 = 1.0 / t207;
        let t209 = t80 * t208;
        let t213 = -t34 * sigma2 * t190 * t85 / 9.0 + t197 * t203 / 216.0 + t136 * t195 * t201 * t209 / 216.0;
        let t218 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t184 * t89 - t163 - 3.0 / 8.0 * t5 * t69 * t213);
        let tvrho1 = t59 + t93 + t6 * (t178 + t218);
        vrho[ip * 2 + 1] += tvrho1;
        let t227 = t126 * rho0;
        let t229 = 1.0 / t36 / t227;
        let t231 = t229 * param_alpha * t51;
        let t238 = t34 * t39 * t46 * t50 / 24.0 - t120 * t122 * sigma0 * t231 / 576.0 - t136 * sigma0 * t229 * t140 / 576.0;
        let t242 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t238);
        let tvsigma0 = t6 * t242;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t249 = t198 * rho1;
        let t251 = 1.0 / t71 / t249;
        let t253 = t251 * param_alpha * t85;
        let t260 = t34 * t74 * t80 * t84 / 24.0 - t120 * t122 * sigma2 * t253 / 576.0 - t136 * sigma2 * t251 * t209 / 576.0;
        let t264 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t260);
        let tvsigma2 = t6 * t264;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
