//! GGA_X_HTBS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_htbs.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_htbs_vxc_unpol(
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
        let t25 = t21 / t23;
        let t26 = rmath::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t32 = t25 * t28 * t30;
        let t33 = t32 / 12.0;
        let t34 = t33 <= 0.6;
        let t35 = t23 * t23;
        let t36 = 1.0 / t35;
        let t37 = t20 * t36;
        let t38 = t27 * t27;
        let t39 = sigma[ip] * t38;
        let t40 = rho[ip] * rho[ip];
        let t41 = t18 * t18;
        let t43 = 1.0 / t41 / t40;
        let t45 = t37 * t39 * t43;
        let t47 = t37 * sigma[ip];
        let t48 = t38 * t43;
        let t50 = rmath::exp(-t45 / 24.0);
        let t51 = t48 * t50;
        let t55 = 1.0 / t23 / t22;
        let t56 = t21 * t55;
        let t57 = sigma[ip] * sigma[ip];
        let t58 = t57 * t27;
        let t59 = t40 * t40;
        let t60 = t59 * rho[ip];
        let t62 = 1.0 / t18 / t60;
        let t64 = t56 * t58 * t62;
        let t66 = 1.0 + 2.7560657413756314e-05 * t64;
        let t67 = rmath::ln(t66);
        let t68 = 0.804 + 5.0 / 972.0 * t45 + 0.004002424276710846 * t47 * t51 + t67;
        let t71 = 1.804 - 0.646416 / t68;
        let t72 = 2.6 <= t33;
        let t74 = rmath::exp(-0.011376190545424806 * t45);
        let t76 = 1.804 - 0.804 * t74;
        let t77 = 0.190125 * t32;
        let t78 = 0.195 * t45;
        let t79 = t26 * sigma[ip];
        let t80 = 1.0 / t59;
        let t82 = 0.017625664237781676 * t79 * t80;
        let t83 = 0.005208333333333333 * t64;
        let t86 = t20 / t35 / t22;
        let t87 = t26 * t57;
        let t88 = t87 * t38;
        let t89 = t59 * t40;
        let t91 = 1.0 / t41 / t89;
        let t94 = 0.0003255208333333333 * t86 * t88 * t91;
        let t95 = -0.40608 + t77 - t78 + t82 - t83 + t94;
        let t97 = 1.40608 - t77 + t78 - t82 + t83 - t94;
        let t100 = piecewise5(t34, t71, t72, t76, t97 * t71 + t95 * t76);
        let t104 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t100);
        let tzk0 = 2.0 * t104;
        zk[ip] += tzk0;
        let t106 = t17 / t41;
        let t110 = t68 * t68;
        let t111 = 1.0 / t110;
        let t112 = t40 * rho[ip];
        let t114 = 1.0 / t41 / t112;
        let t116 = t37 * t39 * t114;
        let t118 = t38 * t114;
        let t119 = t118 * t50;
        let t122 = t56 * t57;
        let t124 = 1.0 / t18 / t89;
        let t125 = t27 * t124;
        let t126 = t125 * t50;
        let t129 = 1.0 / t66;
        let t130 = t125 * t129;
        let t133 = -10.0 / 729.0 * t116 - 0.010673131404562256 * t47 * t119 + 0.0008894276170468547 * t122 * t126 - 0.00014699017287336702 * t122 * t130;
        let t136 = t118 * t74;
        let t140 = 1.0 / t18 / t40;
        let t145 = 1.0 / t60;
        let t151 = t59 * t112;
        let t153 = 1.0 / t41 / t151;
        let t157 = -0.2535 * t25 * t28 * t140 + 0.52 * t116 - 0.0705026569511267 * t79 * t145 + 0.027777777777777776 * t56 * t58 * t124 - 0.002170138888888889 * t86 * t88 * t153;
        let t160 = t95 * t20 * t36;
        let t162 = t39 * t114 * t74;
        let t165 = -t157;
        let t167 = t97 * t111;
        let t171 = piecewise5(t34, 0.646416 * t111 * t133, t72, -0.024390552529390784 * t47 * t136, t157 * t76 - 0.024390552529390784 * t160 * t162 + t165 * t71 + 0.646416 * t167 * t133);
        let t176 = piecewise3(t2, 0.0, -t6 * t106 * t100 / 8.0 - 3.0 / 8.0 * t6 * t19 * t171);
        let tvrho0 = 2.0 * rho[ip] * t176 + 2.0 * t104;
        vrho[ip] += tvrho0;
        let t179 = t37 * t48;
        let t183 = t56 * sigma[ip];
        let t184 = t27 * t62;
        let t185 = t184 * t50;
        let t188 = t184 * t129;
        let t191 = 5.0 / 972.0 * t179 + 0.004002424276710846 * t37 * t51 - 0.0003335353563925705 * t183 * t185 + 5.512131482751263e-05 * t183 * t188;
        let t194 = t48 * t74;
        let t197 = 1.0 / t26;
        let t198 = t197 * t27;
        let t205 = sigma[ip] * t27;
        let t209 = t79 * t38;
        let t213 = 0.0950625 * t25 * t198 * t30 - 0.195 * t179 + 0.026438496356672513 * t26 * t80 - 0.010416666666666666 * t56 * t205 * t62 + 0.0008138020833333334 * t86 * t209 * t91;
        let t217 = -t213;
        let t222 = piecewise5(t34, 0.646416 * t111 * t191, t72, 0.009146457198521543 * t37 * t194, t213 * t76 + 0.009146457198521543 * t160 * t194 + t217 * t71 + 0.646416 * t167 * t191);
        let t226 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t222);
        let tvsigma0 = 2.0 * rho[ip] * t226;
        vsigma[ip] += tvsigma0;
    }
}
