//! GGA_K_LLP vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 66 shared lines across all orders.
//! Delta: 70 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_llp_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (66 lines) ---
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
        let t32 = param_beta * t3;
        let t34 = pow_1_3(1.0 / M_PI);
        let t35 = 1.0 / t34;
        let t36 = t32 * t35;
        let t37 = M_CBRT4;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = param_gamma * param_beta;
        let t45 = f64::sqrt(sigma0);
        let t47 = 1.0 / t40 / rho0;
        let t48 = t45 * t47;
        let t49 = f64::ln(t48 + f64::sqrt(t48 * t48 + 1.0));
        let t52 = 1.0 + t44 * t48 * t49;
        let t53 = 1.0 / t52;
        let t58 = 1.0 + 2.0 / 9.0 * t36 * t38 * t43 * t53;
        let t62 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t58);
        let t63 = rho1 <= dens_threshold;
        let t64 = -t17;
        let t66 = piecewise5(t15, t12, t11, t16, t64 * t8);
        let t67 = 1.0 + t66;
        let t68 = t67 <= zeta_threshold;
        let t69 = pow_1_3(t67);
        let t70 = t69 * t69;
        let t72 = piecewise3(t68, t24, t70 * t67);
        let t73 = t72 * t30;
        let t74 = t37 * sigma2;
        let t75 = rho1 * rho1;
        let t76 = pow_1_3(rho1);
        let t77 = t76 * t76;
        let t79 = 1.0 / t77 / t75;
        let t80 = f64::sqrt(sigma2);
        let t82 = 1.0 / t76 / rho1;
        let t83 = t80 * t82;
        let t84 = f64::ln(t83 + f64::sqrt(t83 * t83 + 1.0));
        let t87 = 1.0 + t44 * t83 * t84;
        let t88 = 1.0 / t87;
        let t93 = 1.0 + 2.0 / 9.0 * t36 * t74 * t79 * t88;
        let t97 = piecewise3(t63, 0.0, 3.0 / 20.0 * t6 * t73 * t93);
        let tzk0 = t62 + t97;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (70 lines) ---
        let t98 = t7 * t7;
        let t99 = 1.0 / t98;
        let t100 = t17 * t99;
        let t102 = piecewise5(t11, 0.0, t15, 0.0, t8 - t100);
        let t105 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t102);
        let t106 = t105 * t30;
        let t110 = 1.0 / t29;
        let t111 = t28 * t110;
        let t114 = t6 * t111 * t58 / 10.0;
        let t115 = t39 * rho0;
        let t117 = 1.0 / t41 / t115;
        let t122 = t35 * t37;
        let t123 = t32 * t122;
        let t124 = sigma0 * t43;
        let t125 = t52 * t52;
        let t126 = 1.0 / t125;
        let t128 = 1.0 / t40 / t39;
        let t132 = sigma0 * t117;
        let t133 = t124 + 1.0;
        let t134 = f64::sqrt(t133);
        let t135 = 1.0 / t134;
        let t139 = -4.0 / 3.0 * t44 * t45 * t128 * t49 - 4.0 / 3.0 * t44 * t132 * t135;
        let t140 = t126 * t139;
        let t144 = -16.0 / 27.0 * t36 * t38 * t117 * t53 - 2.0 / 9.0 * t123 * t124 * t140;
        let t149 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t106 * t58 + t114 + 3.0 / 20.0 * t6 * t31 * t144);
        let t150 = t64 * t99;
        let t152 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t150);
        let t155 = piecewise3(t68, 0.0, 5.0 / 3.0 * t70 * t152);
        let t156 = t155 * t30;
        let t160 = t72 * t110;
        let t163 = t6 * t160 * t93 / 10.0;
        let t165 = piecewise3(t63, 0.0, 3.0 / 20.0 * t6 * t156 * t93 + t163);
        let tvrho0 = t62 + t97 + t7 * (t149 + t165);
        vrho[ip * 2] += tvrho0;
        let t169 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t100);
        let t172 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t169);
        let t173 = t172 * t30;
        let t178 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t173 * t58 + t114);
        let t180 = piecewise5(t15, 0.0, t11, 0.0, t8 - t150);
        let t183 = piecewise3(t68, 0.0, 5.0 / 3.0 * t70 * t180);
        let t184 = t183 * t30;
        let t188 = t75 * rho1;
        let t190 = 1.0 / t77 / t188;
        let t195 = sigma2 * t79;
        let t196 = t87 * t87;
        let t197 = 1.0 / t196;
        let t199 = 1.0 / t76 / t75;
        let t203 = sigma2 * t190;
        let t204 = t195 + 1.0;
        let t205 = f64::sqrt(t204);
        let t206 = 1.0 / t205;
        let t210 = -4.0 / 3.0 * t44 * t80 * t199 * t84 - 4.0 / 3.0 * t44 * t203 * t206;
        let t211 = t197 * t210;
        let t215 = -16.0 / 27.0 * t36 * t74 * t190 * t88 - 2.0 / 9.0 * t123 * t195 * t211;
        let t220 = piecewise3(t63, 0.0, 3.0 / 20.0 * t6 * t184 * t93 + t163 + 3.0 / 20.0 * t6 * t73 * t215);
        let tvrho1 = t62 + t97 + t7 * (t178 + t220);
        vrho[ip * 2 + 1] += tvrho1;
        let t223 = t37 * t43;
        let t226 = 1.0 / t45;
        let t233 = t44 * t226 * t47 * t49 / 2.0 + t44 * t43 * t135 / 2.0;
        let t234 = t126 * t233;
        let t238 = -2.0 / 9.0 * t123 * t124 * t234 + 2.0 / 9.0 * t36 * t223 * t53;
        let t242 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t238);
        let tvsigma0 = t7 * t242;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t243 = t37 * t79;
        let t246 = 1.0 / t80;
        let t253 = t44 * t246 * t82 * t84 / 2.0 + t44 * t79 * t206 / 2.0;
        let t254 = t197 * t253;
        let t258 = -2.0 / 9.0 * t123 * t195 * t254 + 2.0 / 9.0 * t36 * t243 * t88;
        let t262 = piecewise3(t63, 0.0, 3.0 / 20.0 * t6 * t73 * t258);
        let tvsigma2 = t7 * t262;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
