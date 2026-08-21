//! MGGA_C_CCALDA vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ccalda_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = 1.0 + param_c;
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / rho[ip];
        let t8 = rho[ip] * rho[ip];
        let t10 = 1.0 / t4 / t8;
        let t13 = tau[ip] * t6 - sigma[ip] * t10 / 8.0;
        let t14 = t2 * t13;
        let t15 = M_CBRT6;
        let t16 = t14 * t15;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t26 = t15 * t20 * t22;
        let t29 = 1.0 + 5.0 / 9.0 * param_c * t13 * t26;
        let t30 = 1.0 / t29;
        let t31 = M_CBRT3;
        let t32 = 1.0 / M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t39 = t34 * t36 / t3;
        let t41 = 1.0 + 0.053425 * t39;
        let t42 = rmath::sqrt(t39);
        let t45 = pow_3_2(t39);
        let t47 = t31 * t31;
        let t48 = t33 * t33;
        let t49 = t47 * t48;
        let t52 = t49 * t35 / t4;
        let t54 = 3.79785 * t42 + 0.8969 * t39 + 0.204775 * t45 + 0.123235 * t52;
        let t57 = 1.0 + 16.081979498692537 / t54;
        let t58 = rmath::ln(t57);
        let t62 = pow_1_3(zeta_threshold);
        let t64 = piecewise3(1.0 <= zeta_threshold, t62 * zeta_threshold, 1.0);
        let t70 = (2.0 * t64 - 2.0) / (2.0 * t21 - 2.0);
        let t72 = 1.0 + 0.0278125 * t39;
        let t77 = 5.1785 * t42 + 0.905775 * t39 + 0.1100325 * t45 + 0.1241775 * t52;
        let t80 = 1.0 + 29.608749977793437 / t77;
        let t81 = rmath::ln(t80);
        let t85 = -0.0621814 * t41 * t58 + 0.0197516734986138 * t70 * t72 * t81;
        let t87 = t23 * t30 * t85;
        let t89 = 5.0 / 9.0 * t16 * t87;
        let t90 = t23 * t30;
        let t93 = 1.0 - 5.0 / 9.0 * t16 * t90;
        let t94 = t93 * t85;
        let tzk0 = t89 + t94;
        zk[ip] += tzk0;
        let t97 = t8 * rho[ip];
        let t99 = 1.0 / t4 / t97;
        let t102 = -5.0 / 3.0 * tau[ip] * t10 + sigma[ip] * t99 / 3.0;
        let t103 = t2 * t102;
        let t104 = t103 * t15;
        let t105 = t104 * t87;
        let t107 = t15 * t15;
        let t109 = 1.0 / t18 / t17;
        let t110 = t107 * t109;
        let t111 = t14 * t110;
        let t112 = t29 * t29;
        let t113 = 1.0 / t112;
        let t114 = t21 * t113;
        let t115 = t85 * param_c;
        let t117 = t114 * t115 * t102;
        let t118 = t111 * t117;
        let t121 = 1.0 / t3 / rho[ip];
        let t122 = t36 * t121;
        let t126 = t54 * t54;
        let t127 = 1.0 / t126;
        let t128 = t41 * t127;
        let t130 = 1.0 / t42 * t31;
        let t131 = t33 * t36;
        let t132 = t131 * t121;
        let t133 = t130 * t132;
        let t135 = t34 * t122;
        let t137 = rmath::sqrt(t39);
        let t138 = t137 * t31;
        let t139 = t138 * t132;
        let t142 = t49 * t35 * t6;
        let t144 = -0.632975 * t133 - 0.29896666666666666 * t135 - 0.1023875 * t139 - 0.08215666666666667 * t142;
        let t145 = 1.0 / t57;
        let t146 = t144 * t145;
        let t149 = t70 * t31;
        let t154 = t70 * t72;
        let t155 = t77 * t77;
        let t156 = 1.0 / t155;
        let t161 = -0.8630833333333333 * t133 - 0.301925 * t135 - 0.05501625 * t139 - 0.082785 * t142;
        let t163 = 1.0 / t80;
        let t164 = t156 * t161 * t163;
        let t167 = 0.0011073470983333333 * t34 * t122 * t58 + 1.0 * t128 * t146 - 0.00018311447306006544 * t149 * t131 * t121 * t81 - 0.5848223622634646 * t154 * t164;
        let t169 = t23 * t30 * t167;
        let t170 = t16 * t169;
        let t175 = t114 * param_c * t102;
        let t178 = -5.0 / 9.0 * t104 * t90 + 50.0 / 81.0 * t111 * t175;
        let t179 = t178 * t85;
        let t180 = t93 * t167;
        let tvrho0 = t89 + t94 + rho[ip] * (5.0 / 9.0 * t105 - 50.0 / 81.0 * t118 + 5.0 / 9.0 * t170 + t179 + t180);
        vrho[ip] += tvrho0;
        let t183 = t2 * t10;
        let t184 = t183 * t15;
        let t185 = t184 * t87;
        let t186 = 5.0 / 72.0 * t185;
        let t189 = t111 * t114 * t115 * t10;
        let t190 = 25.0 / 324.0 * t189;
        let t191 = t184 * t90;
        let t195 = t111 * t114 * param_c * t10;
        let t197 = 5.0 / 72.0 * t191 - 25.0 / 324.0 * t195;
        let t198 = t197 * t85;
        let tvsigma0 = rho[ip] * (-t186 + t190 + t198);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t200 = t2 * t6;
        let t201 = t200 * t15;
        let t203 = 5.0 / 9.0 * t201 * t87;
        let t207 = 50.0 / 81.0 * t111 * t114 * t115 * t6;
        let t214 = -5.0 / 9.0 * t201 * t90 + 50.0 / 81.0 * t111 * t114 * param_c * t6;
        let t215 = t214 * t85;
        let tvtau0 = rho[ip] * (t203 - t207 + t215);
        vtau[ip] += tvtau0;
    }
}
