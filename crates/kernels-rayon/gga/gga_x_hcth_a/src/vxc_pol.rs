//! GGA_X_HCTH_A vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hcth_a.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hcth_a_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t32 = t28 / t30;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t41 = rmath::sqrt(sigma0);
        let t43 = 1.0 / t36 / rho0;
        let t44 = t41 * t43;
        let t45 = rmath::ln(t44 + rmath::sqrt(t44 * t44 + 1.0));
        let t48 = 1.0 + 0.0252 * t44 * t45;
        let t51 = t48 * t48;
        let t52 = 1.0 / t51;
        let t54 = -2.51173 / t48 + 3.7198333333333333 * t52;
        let t58 = 1.09878 + 0.0009333333333333333 * t34 * t40 * t54;
        let t62 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t58);
        let t63 = rho1 <= dens_threshold;
        let t64 = -t16;
        let t66 = piecewise5(t14, t11, t10, t15, t64 * t7);
        let t67 = 1.0 + t66;
        let t68 = t67 <= zeta_threshold;
        let t69 = pow_1_3(t67);
        let t71 = piecewise3(t68, t22, t69 * t67);
        let t72 = t71 * t26;
        let t73 = rho1 * rho1;
        let t74 = pow_1_3(rho1);
        let t75 = t74 * t74;
        let t77 = 1.0 / t75 / t73;
        let t78 = sigma2 * t77;
        let t79 = rmath::sqrt(sigma2);
        let t81 = 1.0 / t74 / rho1;
        let t82 = t79 * t81;
        let t83 = rmath::ln(t82 + rmath::sqrt(t82 * t82 + 1.0));
        let t86 = 1.0 + 0.0252 * t82 * t83;
        let t89 = t86 * t86;
        let t90 = 1.0 / t89;
        let t92 = -2.51173 / t86 + 3.7198333333333333 * t90;
        let t96 = 1.09878 + 0.0009333333333333333 * t34 * t78 * t92;
        let t100 = piecewise3(t63, 0.0, -3.0 / 8.0 * t5 * t72 * t96);
        let tzk0 = t62 + t100;
        zk[ip] += tzk0;
        let t101 = t6 * t6;
        let t102 = 1.0 / t101;
        let t103 = t16 * t102;
        let t105 = piecewise5(t10, 0.0, t14, 0.0, t7 - t103);
        let t108 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t105);
        let t109 = t108 * t26;
        let t113 = t26 * t26;
        let t114 = 1.0 / t113;
        let t115 = t25 * t114;
        let t118 = t5 * t115 * t58 / 8.0;
        let t119 = t35 * rho0;
        let t121 = 1.0 / t37 / t119;
        let t122 = sigma0 * t121;
        let t127 = 1.0 / t36 / t35;
        let t131 = t40 + 1.0;
        let t132 = rmath::sqrt(t131);
        let t133 = 1.0 / t132;
        let t136 = -0.0336 * t41 * t127 * t45 - 0.0336 * t122 * t133;
        let t140 = 1.0 / t51 / t48;
        let t141 = t140 * t136;
        let t143 = 2.51173 * t52 * t136 - 7.439666666666667 * t141;
        let t147 = -0.002488888888888889 * t34 * t122 * t54 + 0.0009333333333333333 * t34 * t40 * t143;
        let t152 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t109 * t58 - t118 - 3.0 / 8.0 * t5 * t27 * t147);
        let t153 = t64 * t102;
        let t155 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t153);
        let t158 = piecewise3(t68, 0.0, 4.0 / 3.0 * t69 * t155);
        let t159 = t158 * t26;
        let t163 = t71 * t114;
        let t166 = t5 * t163 * t96 / 8.0;
        let t168 = piecewise3(t63, 0.0, -3.0 / 8.0 * t5 * t159 * t96 - t166);
        let tvrho0 = t62 + t100 + t6 * (t152 + t168);
        vrho[ip * 2] += tvrho0;
        let t172 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t103);
        let t175 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t172);
        let t176 = t175 * t26;
        let t181 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t176 * t58 - t118);
        let t183 = piecewise5(t14, 0.0, t10, 0.0, t7 - t153);
        let t186 = piecewise3(t68, 0.0, 4.0 / 3.0 * t69 * t183);
        let t187 = t186 * t26;
        let t191 = t73 * rho1;
        let t193 = 1.0 / t75 / t191;
        let t194 = sigma2 * t193;
        let t199 = 1.0 / t74 / t73;
        let t203 = t78 + 1.0;
        let t204 = rmath::sqrt(t203);
        let t205 = 1.0 / t204;
        let t208 = -0.0336 * t79 * t199 * t83 - 0.0336 * t194 * t205;
        let t212 = 1.0 / t89 / t86;
        let t213 = t212 * t208;
        let t215 = 2.51173 * t90 * t208 - 7.439666666666667 * t213;
        let t219 = -0.002488888888888889 * t34 * t194 * t92 + 0.0009333333333333333 * t34 * t78 * t215;
        let t224 = piecewise3(t63, 0.0, -3.0 / 8.0 * t5 * t187 * t96 - t166 - 3.0 / 8.0 * t5 * t72 * t219);
        let tvrho1 = t62 + t100 + t6 * (t181 + t224);
        vrho[ip * 2 + 1] += tvrho1;
        let t227 = t33 * t39;
        let t231 = 1.0 / t41;
        let t237 = 0.0126 * t231 * t43 * t45 + 0.0126 * t39 * t133;
        let t240 = t140 * t237;
        let t242 = 2.51173 * t52 * t237 - 7.439666666666667 * t240;
        let t246 = 0.0009333333333333333 * t32 * t227 * t54 + 0.0009333333333333333 * t34 * t40 * t242;
        let t250 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t246);
        let tvsigma0 = t6 * t250;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t251 = t33 * t77;
        let t255 = 1.0 / t79;
        let t261 = 0.0126 * t255 * t81 * t83 + 0.0126 * t77 * t205;
        let t264 = t212 * t261;
        let t266 = 2.51173 * t90 * t261 - 7.439666666666667 * t264;
        let t270 = 0.0009333333333333333 * t32 * t251 * t92 + 0.0009333333333333333 * t34 * t78 * t266;
        let t274 = piecewise3(t63, 0.0, -3.0 / 8.0 * t5 * t72 * t270);
        let tvsigma2 = t6 * t274;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
