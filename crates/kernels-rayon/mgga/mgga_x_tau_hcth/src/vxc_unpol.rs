//! MGGA_X_TAU_HCTH vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tau_hcth_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t22 = param_cx_local_1;
        let t23 = t22 * sigma[ip];
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = rho[ip] * rho[ip];
        let t27 = t19 * t19;
        let t29 = 1.0 / t27 / t26;
        let t30 = t25 * t29;
        let t34 = 1.0 + 0.004 * sigma[ip] * t25 * t29;
        let t35 = 1.0 / t34;
        let t36 = t30 * t35;
        let t39 = param_cx_local_2;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t39 * t40;
        let t42 = t26 * t26;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t19 / t43;
        let t46 = t24 * t45;
        let t47 = t34 * t34;
        let t48 = 1.0 / t47;
        let t49 = t46 * t48;
        let t52 = param_cx_local_3;
        let t53 = t40 * sigma[ip];
        let t54 = t52 * t53;
        let t55 = t42 * t42;
        let t56 = 1.0 / t55;
        let t57 = t47 * t34;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t63 = param_cx_nlocal_1;
        let t64 = t63 * sigma[ip];
        let t67 = param_cx_nlocal_2;
        let t68 = t67 * t40;
        let t71 = param_cx_nlocal_3;
        let t72 = t71 * t53;
        let t75 = param_cx_nlocal_0 + 0.004 * t64 * t36 + 3.2e-05 * t68 * t49 + 2.56e-07 * t72 * t59;
        let t76 = M_CBRT6;
        let t77 = t76 * t76;
        let t78 = M_PI * M_PI;
        let t79 = pow_1_3(t78);
        let t80 = t79 * t79;
        let t82 = 3.0 / 10.0 * t77 * t80;
        let t83 = tau[ip] * t25;
        let t85 = 1.0 / t27 / rho[ip];
        let t86 = t83 * t85;
        let t87 = t82 - t86;
        let t88 = t82 + t86;
        let t89 = 1.0 / t88;
        let t91 = t87 * t87;
        let t92 = t91 * t87;
        let t93 = t88 * t88;
        let t94 = t93 * t88;
        let t95 = 1.0 / t94;
        let t98 = t91 * t91;
        let t99 = t98 * t87;
        let t100 = t93 * t93;
        let t102 = 1.0 / t100 / t88;
        let t104 = t99 * t102 + t87 * t89 - 2.0 * t92 * t95;
        let t106 = param_cx_local_0 + 0.004 * t23 * t36 + 3.2e-05 * t41 * t49 + 2.56e-07 * t54 * t59 + t75 * t104;
        let t110 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t106);
        let tzk0 = 2.0 * t110;
        zk[ip] += tzk0;
        let t111 = 1.0 / t27;
        let t112 = t18 * t111;
        let t116 = t26 * rho[ip];
        let t118 = 1.0 / t27 / t116;
        let t119 = t25 * t118;
        let t120 = t119 * t35;
        let t123 = t22 * t40;
        let t124 = t42 * t26;
        let t126 = 1.0 / t19 / t124;
        let t127 = t24 * t126;
        let t128 = t127 * t48;
        let t133 = t39 * t53;
        let t134 = t55 * rho[ip];
        let t135 = 1.0 / t134;
        let t136 = t135 * t58;
        let t141 = t40 * t40;
        let t142 = t52 * t141;
        let t143 = t55 * t116;
        let t145 = 1.0 / t27 / t143;
        let t146 = t47 * t47;
        let t147 = 1.0 / t146;
        let t149 = t145 * t147 * t25;
        let t154 = t63 * t40;
        let t159 = t67 * t53;
        let t164 = t71 * t141;
        let t167 = -0.010666666666666666 * t64 * t120 + 8.533333333333334e-05 * t154 * t128 - 0.00017066666666666668 * t68 * t128 + 1.3653333333333333e-06 * t159 * t136 - 2.048e-06 * t72 * t136 + 8.192e-09 * t164 * t149;
        let t172 = 1.0 / t93;
        let t173 = t87 * t172;
        let t174 = t83 * t29;
        let t177 = t91 * t95;
        let t180 = 1.0 / t100;
        let t181 = t92 * t180;
        let t184 = t98 * t102;
        let t188 = 1.0 / t100 / t93;
        let t189 = t99 * t188;
        let t192 = 5.0 / 3.0 * t83 * t29 * t89 + 5.0 / 3.0 * t173 * t174 - 10.0 * t177 * t174 - 10.0 * t181 * t174 + 25.0 / 3.0 * t184 * t174 + 25.0 / 3.0 * t189 * t174;
        let t194 = -0.010666666666666666 * t23 * t120 + 8.533333333333334e-05 * t123 * t128 - 0.00017066666666666668 * t41 * t128 + 1.3653333333333333e-06 * t133 * t136 - 2.048e-06 * t54 * t136 + 8.192e-09 * t142 * t149 + t167 * t104 + t75 * t192;
        let t199 = piecewise3(t3, 0.0, -t7 * t112 * t106 / 8.0 - 3.0 / 8.0 * t7 * t20 * t194);
        let tvrho0 = 2.0 * rho[ip] * t199 + 2.0 * t110;
        vrho[ip] += tvrho0;
        let t202 = t22 * t25;
        let t203 = t29 * t35;
        let t208 = t39 * sigma[ip];
        let t213 = t52 * t40;
        let t216 = t55 * t26;
        let t218 = 1.0 / t27 / t216;
        let t220 = t218 * t147 * t25;
        let t223 = t63 * t25;
        let t228 = t67 * sigma[ip];
        let t233 = t71 * t40;
        let t238 = 0.004 * t223 * t203 - 3.2e-05 * t64 * t49 + 6.4e-05 * t228 * t49 - 5.12e-07 * t68 * t59 + 7.68e-07 * t233 * t59 - 3.072e-09 * t72 * t220;
        let t240 = 0.004 * t202 * t203 - 3.2e-05 * t23 * t49 + 6.4e-05 * t208 * t49 - 5.12e-07 * t41 * t59 + 7.68e-07 * t213 * t59 - 3.072e-09 * t54 * t220 + t238 * t104;
        let t244 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t240);
        let tvsigma0 = 2.0 * rho[ip] * t244;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t246 = t7 * t18;
        let t247 = t19 * t75;
        let t248 = t25 * t85;
        let t259 = -t173 * t248 + 6.0 * t177 * t248 + 6.0 * t181 * t248 - 5.0 * t184 * t248 - 5.0 * t189 * t248 - t248 * t89;
        let t263 = piecewise3(t3, 0.0, -3.0 / 8.0 * t246 * t247 * t259);
        let tvtau0 = 2.0 * rho[ip] * t263;
        vtau[ip] += tvtau0;
    }
}
