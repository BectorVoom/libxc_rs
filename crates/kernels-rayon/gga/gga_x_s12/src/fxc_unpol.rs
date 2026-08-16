//! GGA_X_S12 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_s12_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t19 * param_bx;
        let t21 = param_C * sigma[ip];
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = t19 * t19;
        let t27 = 1.0 / t25 / t24;
        let t28 = t23 * t27;
        let t30 = sigma[ip] * sigma[ip];
        let t31 = param_D * t30;
        let t32 = t24 * t24;
        let t33 = t32 * rho[ip];
        let t35 = 1.0 / t19 / t33;
        let t36 = t22 * t35;
        let t39 = t21 * t28 + 2.0 * t31 * t36 + 1.0;
        let t42 = param_B * (1.0 - 1.0 / t39);
        let t43 = param_E * sigma[ip];
        let t45 = t43 * t28 + 1.0;
        let t47 = 1.0 - 1.0 / t45;
        let t49 = t42 * t47 + param_A;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t55 = 1.0 / t25 * param_bx;
        let t59 = t39 * t39;
        let t61 = param_B / t59;
        let t62 = t24 * rho[ip];
        let t64 = 1.0 / t25 / t62;
        let t65 = t23 * t64;
        let t68 = t32 * t24;
        let t70 = 1.0 / t19 / t68;
        let t71 = t22 * t70;
        let t74 = -8.0 / 3.0 * t21 * t65 - 32.0 / 3.0 * t31 * t71;
        let t75 = t74 * t47;
        let t77 = t45 * t45;
        let t78 = 1.0 / t77;
        let t79 = t42 * t78;
        let t80 = t43 * t65;
        let t83 = t61 * t75 - 8.0 / 3.0 * t79 * t80;
        let t88 = piecewise3(t2, 0.0, -t18 * t55 * t49 / 8.0 - 3.0 / 8.0 * t18 * t20 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t91 = param_C * t23;
        let t93 = param_D * sigma[ip];
        let t96 = t91 * t27 + 4.0 * t93 * t36;
        let t97 = t96 * t47;
        let t99 = param_E * t23;
        let t102 = t79 * t99 * t27 + t61 * t97;
        let t106 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t102);
        let tvsigma0 = 2.0 * rho[ip] * t106;
        vsigma[ip] += tvsigma0;
        let t111 = 1.0 / t25 / rho[ip] * param_bx;
        let t120 = param_B / t59 / t39;
        let t121 = t74 * t74;
        let t122 = t121 * t47;
        let t126 = 1.0 / t25 / t32;
        let t127 = t23 * t126;
        let t132 = 1.0 / t19 / t32 / t62;
        let t133 = t22 * t132;
        let t136 = 88.0 / 9.0 * t21 * t127 + 608.0 / 9.0 * t31 * t133;
        let t137 = t136 * t47;
        let t139 = t74 * t78;
        let t140 = t61 * t139;
        let t144 = 1.0 / t77 / t45;
        let t145 = t42 * t144;
        let t146 = param_E * param_E;
        let t147 = t146 * t30;
        let t148 = t147 * t133;
        let t151 = t43 * t127;
        let t154 = -2.0 * t120 * t122 + t61 * t137 - 16.0 / 3.0 * t140 * t80 - 256.0 / 9.0 * t145 * t148 + 88.0 / 9.0 * t79 * t151;
        let t159 = piecewise3(t2, 0.0, t18 * t111 * t49 / 12.0 - t18 * t55 * t83 / 4.0 - 3.0 / 8.0 * t18 * t20 * t154);
        let tv2rho20 = 2.0 * rho[ip] * t159 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t172 = -8.0 / 3.0 * t91 * t64 - 64.0 / 3.0 * t93 * t71;
        let t173 = t172 * t47;
        let t175 = t96 * t78;
        let t176 = t61 * t175;
        let t179 = t61 * t74;
        let t180 = t78 * param_E;
        let t181 = t180 * t28;
        let t183 = t146 * t22;
        let t185 = t183 * t70 * sigma[ip];
        let t191 = -2.0 * t120 * t97 * t74 + t61 * t173 - 8.0 / 3.0 * t176 * t80 + t179 * t181 + 32.0 / 3.0 * t145 * t185 - 8.0 / 3.0 * t79 * t99 * t64;
        let t196 = piecewise3(t2, 0.0, -t18 * t55 * t102 / 8.0 - 3.0 / 8.0 * t18 * t20 * t191);
        let tv2rhosigma0 = 2.0 * rho[ip] * t196 + 2.0 * t106;
        v2rhosigma[ip] += tv2rhosigma0;
        let t199 = t96 * t96;
        let t200 = t199 * t47;
        let t203 = t61 * param_D;
        let t207 = t61 * t96;
        let t213 = -4.0 * t145 * t183 * t35 + 4.0 * t203 * t36 * t47 - 2.0 * t120 * t200 + 2.0 * t207 * t181;
        let t217 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t213);
        let tv2sigma20 = 2.0 * rho[ip] * t217;
        v2sigma2[ip] += tv2sigma20;
    }
}
