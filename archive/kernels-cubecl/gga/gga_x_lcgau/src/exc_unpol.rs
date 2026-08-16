//! GGA_X_LCGAU exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lcgau.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lcgau_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_coeff_2: f64,
    param_hyb_coeff_3: f64,
    param_hyb_omega_0: f64,
    param_hyb_omega_2: f64,
    param_hyb_omega_3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t3 * t3;
        let t21 = 1.0 / M_PI;
        let t22 = pow_1_3::<f64>(t21);
        let t24 = t20 / t22;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = f64::sqrt(sigma[ip]);
        let t35 = t34 * t27;
        let t37 = 1.0 / t19 / rho[ip];
        let t39 = f64::ln(t35 * t37 + f64::sqrt(pow_2::<f64>(t35 * t37) + 1.0));
        let t40 = t37 * t39;
        let t43 = 1.0 + 0.252e-1 * t35 * t40;
        let t44 = 1.0 / t43;
        let t49 = 1.0 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t44;
        let t50 = t19 * t49;
        let t51 = f64::sqrt(3.0);
        let t52 = param_hyb_omega_0 * t51;
        let t54 = t25 * t25;
        let t57 = t3 * t22 * t54 * t49 * t21;
        let t58 = f64::sqrt(t57);
        let t60 = t11 * rho[ip];
        let t61 = pow_1_3::<f64>(t60);
        let t62 = 1.0 / t61;
        let t63 = t58 * t27 * t62;
        let t65 = t52 * t63 / 12.0;
        let t66 = 0.135e1 <= t65;
        let t67 = 0.135e1 < t65;
        let t68 = piecewise3::<f64>(t67, t65, 0.135e1);
        let t69 = t68 * t68;
        let t72 = t69 * t69;
        let t73 = 1.0 / t72;
        let t75 = t72 * t69;
        let t76 = 1.0 / t75;
        let t78 = t72 * t72;
        let t79 = 1.0 / t78;
        let t82 = 1.0 / t78 / t69;
        let t85 = 1.0 / t78 / t72;
        let t88 = 1.0 / t78 / t75;
        let t90 = t78 * t78;
        let t91 = 1.0 / t90;
        let t94 = piecewise3::<f64>(t67, 0.135e1, t65);
        let t95 = f64::sqrt(M_PI);
        let t96 = 1.0 / t94;
        let t98 = erf_approx::<f64>(t96 / 2.0);
        let t100 = t94 * t94;
        let t101 = 1.0 / t100;
        let t103 = f64::exp(-t101 / 4.0);
        let t104 = t103 - 1.0;
        let t107 = t103 - 3.0 / 2.0 - 2.0 * t100 * t104;
        let t110 = 2.0 * t94 * t107 + t95 * t98;
        let t114 = piecewise3::<f64>(t66, 1.0 / t69 / 36.0 - t73 / 960.0 + t76 / 26880.0 - t79 / 829440.0 + t82 / 28385280.0 - t85 / 0.107347968e10 + t88 / 0.445906944e11 - t91 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t94 * t110);
        let t115 = param_hyb_omega_2 * t51;
        let t117 = t115 * t63 / 12.0;
        let t118 = 0.207e1 <= t117;
        let t119 = 0.207e1 < t117;
        let t120 = piecewise3::<f64>(t119, t117, 0.207e1);
        let t121 = t120 * t120;
        let t124 = t121 * t121;
        let t125 = 1.0 / t124;
        let t127 = t124 * t121;
        let t128 = 1.0 / t127;
        let t130 = t124 * t124;
        let t131 = 1.0 / t130;
        let t134 = 1.0 / t130 / t121;
        let t137 = 1.0 / t130 / t124;
        let t140 = 1.0 / t130 / t127;
        let t143 = piecewise3::<f64>(t119, 0.207e1, t117);
        let t144 = 1.0 / t143;
        let t146 = erf_approx::<f64>(t144 / 2.0);
        let t148 = t143 * t143;
        let t149 = 1.0 / t148;
        let t151 = f64::exp(-t149 / 4.0);
        let t152 = t151 - 1.0;
        let t153 = t143 * t152;
        let t155 = 1.0 - 8.0 * t148;
        let t159 = t95 * t146 + 2.0 * t153 * t155 - 4.0 * t143;
        let t162 = piecewise3::<f64>(t118, -1.0 / t121 / 18.0 + t125 / 240.0 - t128 / 4480.0 + t131 / 103680.0 - t134 / 2838528.0 + t137 / 89456640.0 - t140 / 0.31850496e10, -8.0 / 3.0 * t143 * t159);
        let t164 = param_hyb_omega_3 * t51;
        let t166 = t164 * t63 / 12.0;
        let t167 = 0.207e1 <= t166;
        let t168 = 0.207e1 < t166;
        let t169 = piecewise3::<f64>(t168, t166, 0.207e1);
        let t170 = t169 * t169;
        let t173 = t170 * t170;
        let t174 = 1.0 / t173;
        let t176 = t173 * t170;
        let t177 = 1.0 / t176;
        let t179 = t173 * t173;
        let t180 = 1.0 / t179;
        let t183 = 1.0 / t179 / t170;
        let t186 = 1.0 / t179 / t173;
        let t189 = 1.0 / t179 / t176;
        let t192 = piecewise3::<f64>(t168, 0.207e1, t166);
        let t193 = 1.0 / t192;
        let t195 = erf_approx::<f64>(t193 / 2.0);
        let t197 = t192 * t192;
        let t198 = 1.0 / t197;
        let t200 = f64::exp(-t198 / 4.0);
        let t201 = t200 - 1.0;
        let t202 = t192 * t201;
        let t204 = 1.0 - 8.0 * t197;
        let t208 = t95 * t195 + 2.0 * t202 * t204 - 4.0 * t192;
        let t211 = piecewise3::<f64>(t167, -1.0 / t170 / 18.0 + t174 / 240.0 - t177 / 4480.0 + t180 / 103680.0 - t183 / 2838528.0 + t186 / 89456640.0 - t189 / 0.31850496e10, -8.0 / 3.0 * t192 * t208);
        let t213 = param_hyb_coeff_2 * t162 + param_hyb_coeff_3 * t211 + t114;
        let t217 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t50 * t213);
        let tzk0 = 2.0 * t217;
        zk[ip] += tzk0;
    }
}
