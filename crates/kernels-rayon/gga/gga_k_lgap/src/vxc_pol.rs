//! GGA_K_LGAP vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_kappa: f64,
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
        let t39 = f64::sqrt(sigma0);
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
        let t69 = f64::exp(-t35 * t40 * t43 / 12.0 - t48 * t51 * t55 / 24.0 - t61 * t62 * t64 / 48.0);
        let t72 = 1.0 + param_kappa * (1.0 - t69);
        let t76 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t72);
        let t77 = rho1 <= dens_threshold;
        let t78 = -t17;
        let t80 = piecewise5(t15, t12, t11, t16, t78 * t8);
        let t81 = 1.0 + t80;
        let t82 = t81 <= zeta_threshold;
        let t83 = pow_1_3(t81);
        let t84 = t83 * t83;
        let t86 = piecewise3(t82, t24, t84 * t81);
        let t87 = t86 * t30;
        let t88 = f64::sqrt(sigma2);
        let t89 = t38 * t88;
        let t90 = pow_1_3(rho1);
        let t92 = 1.0 / t90 / rho1;
        let t96 = t50 * sigma2;
        let t97 = rho1 * rho1;
        let t98 = t90 * t90;
        let t100 = 1.0 / t98 / t97;
        let t104 = t88 * sigma2;
        let t105 = t97 * t97;
        let t106 = 1.0 / t105;
        let t111 = f64::exp(-t35 * t89 * t92 / 12.0 - t48 * t96 * t100 / 24.0 - t61 * t104 * t106 / 48.0);
        let t114 = 1.0 + param_kappa * (1.0 - t111);
        let t118 = piecewise3(t77, 0.0, 3.0 / 20.0 * t6 * t87 * t114);
        let tzk0 = t76 + t118;
        zk[ip] += tzk0;
        let t119 = t7 * t7;
        let t120 = 1.0 / t119;
        let t121 = t17 * t120;
        let t123 = piecewise5(t11, 0.0, t15, 0.0, t8 - t121);
        let t126 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t123);
        let t127 = t126 * t30;
        let t131 = 1.0 / t29;
        let t132 = t28 * t131;
        let t135 = t6 * t132 * t72 / 10.0;
        let t136 = t6 * t28;
        let t137 = t30 * param_kappa;
        let t139 = 1.0 / t41 / t52;
        let t143 = t52 * rho0;
        let t145 = 1.0 / t53 / t143;
        let t149 = t63 * rho0;
        let t150 = 1.0 / t149;
        let t154 = t35 * t40 * t139 / 9.0 + t48 * t51 * t145 / 9.0 + t61 * t62 * t150 / 12.0;
        let t155 = t154 * t69;
        let t156 = t137 * t155;
        let t160 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t127 * t72 + t135 - 3.0 / 20.0 * t136 * t156);
        let t161 = t78 * t120;
        let t163 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t161);
        let t166 = piecewise3(t82, 0.0, 5.0 / 3.0 * t84 * t163);
        let t167 = t166 * t30;
        let t171 = t86 * t131;
        let t174 = t6 * t171 * t114 / 10.0;
        let t176 = piecewise3(t77, 0.0, 3.0 / 20.0 * t6 * t167 * t114 + t174);
        let tvrho0 = t76 + t118 + t7 * (t160 + t176);
        vrho[ip * 2] += tvrho0;
        let t180 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t121);
        let t183 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t180);
        let t184 = t183 * t30;
        let t189 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t184 * t72 + t135);
        let t191 = piecewise5(t15, 0.0, t11, 0.0, t8 - t161);
        let t194 = piecewise3(t82, 0.0, 5.0 / 3.0 * t84 * t191);
        let t195 = t194 * t30;
        let t199 = t6 * t86;
        let t201 = 1.0 / t90 / t97;
        let t205 = t97 * rho1;
        let t207 = 1.0 / t98 / t205;
        let t211 = t105 * rho1;
        let t212 = 1.0 / t211;
        let t216 = t35 * t89 * t201 / 9.0 + t48 * t96 * t207 / 9.0 + t61 * t104 * t212 / 12.0;
        let t217 = t216 * t111;
        let t218 = t137 * t217;
        let t222 = piecewise3(t77, 0.0, 3.0 / 20.0 * t6 * t195 * t114 + t174 - 3.0 / 20.0 * t199 * t218);
        let tvrho1 = t76 + t118 + t7 * (t189 + t222);
        vrho[ip * 2 + 1] += tvrho1;
        let t225 = 1.0 / t39;
        let t226 = t38 * t225;
        let t236 = -t35 * t226 * t43 / 24.0 - t48 * t50 * t55 / 24.0 - t61 * t39 * t64 / 32.0;
        let t237 = t236 * t69;
        let t238 = t137 * t237;
        let t241 = piecewise3(t1, 0.0, -3.0 / 20.0 * t136 * t238);
        let tvsigma0 = t7 * t241;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t242 = 1.0 / t88;
        let t243 = t38 * t242;
        let t253 = -t35 * t243 * t92 / 24.0 - t48 * t50 * t100 / 24.0 - t61 * t88 * t106 / 32.0;
        let t254 = t253 * t111;
        let t255 = t137 * t254;
        let t258 = piecewise3(t77, 0.0, -3.0 / 20.0 * t199 * t255);
        let tvsigma2 = t7 * t258;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
