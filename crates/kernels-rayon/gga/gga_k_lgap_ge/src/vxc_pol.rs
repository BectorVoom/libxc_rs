//! GGA_K_LGAP_GE vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap_ge.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_ge_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
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
        let t33 = M_CBRT6;
        let t34 = t33 * t33;
        let t35 = param_mu_0 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = 1.0 / t37;
        let t39 = rmath::sqrt(sigma0);
        let t40 = t38 * t39;
        let t41 = pow_1_3(rho0);
        let t43 = 1.0 / t41 / rho0;
        let t48 = param_mu_1 * t33;
        let t49 = t37 * t37;
        let t50 = 1.0 / t49;
        let t51 = t50 * sigma0;
        let t52 = rho0 * rho0;
        let t53 = t41 * t41;
        let t55 = 1.0 / t53 / t52;
        let t61 = param_mu_2 / t36;
        let t62 = t39 * sigma0;
        let t63 = t52 * t52;
        let t64 = 1.0 / t63;
        let t68 = 1.0 + t35 * t40 * t43 / 12.0 + t48 * t51 * t55 / 24.0 + t61 * t62 * t64 / 48.0;
        let t72 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t68);
        let t73 = rho1 <= dens_threshold;
        let t74 = -t17;
        let t76 = piecewise5(t15, t12, t11, t16, t74 * t8);
        let t77 = 1.0 + t76;
        let t78 = t77 <= zeta_threshold;
        let t79 = pow_1_3(t77);
        let t80 = t79 * t79;
        let t82 = piecewise3(t78, t24, t80 * t77);
        let t83 = t82 * t30;
        let t84 = rmath::sqrt(sigma2);
        let t85 = t38 * t84;
        let t86 = pow_1_3(rho1);
        let t88 = 1.0 / t86 / rho1;
        let t92 = t50 * sigma2;
        let t93 = rho1 * rho1;
        let t94 = t86 * t86;
        let t96 = 1.0 / t94 / t93;
        let t100 = t84 * sigma2;
        let t101 = t93 * t93;
        let t102 = 1.0 / t101;
        let t106 = 1.0 + t35 * t85 * t88 / 12.0 + t48 * t92 * t96 / 24.0 + t61 * t100 * t102 / 48.0;
        let t110 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t83 * t106);
        let tzk0 = t72 + t110;
        zk[ip] += tzk0;
        let t111 = t7 * t7;
        let t112 = 1.0 / t111;
        let t113 = t17 * t112;
        let t115 = piecewise5(t11, 0.0, t15, 0.0, t8 - t113);
        let t118 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t115);
        let t119 = t118 * t30;
        let t123 = 1.0 / t29;
        let t124 = t28 * t123;
        let t127 = t6 * t124 * t68 / 10.0;
        let t129 = 1.0 / t41 / t52;
        let t133 = t52 * rho0;
        let t135 = 1.0 / t53 / t133;
        let t139 = t63 * rho0;
        let t140 = 1.0 / t139;
        let t144 = -t35 * t40 * t129 / 9.0 - t48 * t51 * t135 / 9.0 - t61 * t62 * t140 / 12.0;
        let t149 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t119 * t68 + t127 + 3.0 / 20.0 * t6 * t31 * t144);
        let t150 = t74 * t112;
        let t152 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t150);
        let t155 = piecewise3(t78, 0.0, 5.0 / 3.0 * t80 * t152);
        let t156 = t155 * t30;
        let t160 = t82 * t123;
        let t163 = t6 * t160 * t106 / 10.0;
        let t165 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t156 * t106 + t163);
        let tvrho0 = t72 + t110 + t7 * (t149 + t165);
        vrho[ip * 2] += tvrho0;
        let t169 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t113);
        let t172 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t169);
        let t173 = t172 * t30;
        let t178 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t173 * t68 + t127);
        let t180 = piecewise5(t15, 0.0, t11, 0.0, t8 - t150);
        let t183 = piecewise3(t78, 0.0, 5.0 / 3.0 * t80 * t180);
        let t184 = t183 * t30;
        let t189 = 1.0 / t86 / t93;
        let t193 = t93 * rho1;
        let t195 = 1.0 / t94 / t193;
        let t199 = t101 * rho1;
        let t200 = 1.0 / t199;
        let t204 = -t35 * t85 * t189 / 9.0 - t48 * t92 * t195 / 9.0 - t61 * t100 * t200 / 12.0;
        let t209 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t184 * t106 + t163 + 3.0 / 20.0 * t6 * t83 * t204);
        let tvrho1 = t72 + t110 + t7 * (t178 + t209);
        vrho[ip * 2 + 1] += tvrho1;
        let t212 = 1.0 / t39;
        let t213 = t38 * t212;
        let t223 = t35 * t213 * t43 / 24.0 + t48 * t50 * t55 / 24.0 + t61 * t39 * t64 / 32.0;
        let t227 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t223);
        let tvsigma0 = t7 * t227;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t228 = 1.0 / t84;
        let t229 = t38 * t228;
        let t239 = t35 * t229 * t88 / 24.0 + t48 * t50 * t96 / 24.0 + t61 * t84 * t102 / 32.0;
        let t243 = piecewise3(t73, 0.0, 3.0 / 20.0 * t6 * t83 * t239);
        let tvsigma2 = t7 * t243;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
