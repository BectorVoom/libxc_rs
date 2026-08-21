//! GGA_K_PEARSON vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pearson.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pearson_vxc_pol(
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
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t44 = t33 * t33;
        let t45 = 1.0 / t44;
        let t46 = sigma0 * sigma0;
        let t47 = t46 * sigma0;
        let t49 = t38 * t38;
        let t50 = t49 * t49;
        let t54 = 1.0 + t45 * t47 / t50 / 2304.0;
        let t55 = 1.0 / t54;
        let t59 = 1.0 + 5.0 / 648.0 * t37 * sigma0 * t42 * t55;
        let t63 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t59);
        let t64 = rho1 <= dens_threshold;
        let t65 = -t17;
        let t67 = piecewise5(t15, t12, t11, t16, t65 * t8);
        let t68 = 1.0 + t67;
        let t69 = t68 <= zeta_threshold;
        let t70 = pow_1_3(t68);
        let t71 = t70 * t70;
        let t73 = piecewise3(t69, t24, t71 * t68);
        let t74 = t73 * t30;
        let t75 = rho1 * rho1;
        let t76 = pow_1_3(rho1);
        let t77 = t76 * t76;
        let t79 = 1.0 / t77 / t75;
        let t81 = sigma2 * sigma2;
        let t82 = t81 * sigma2;
        let t84 = t75 * t75;
        let t85 = t84 * t84;
        let t89 = 1.0 + t45 * t82 / t85 / 2304.0;
        let t90 = 1.0 / t89;
        let t94 = 1.0 + 5.0 / 648.0 * t37 * sigma2 * t79 * t90;
        let t98 = piecewise3(t64, 0.0, 3.0 / 20.0 * t6 * t74 * t94);
        let tzk0 = t63 + t98;
        zk[ip] += tzk0;
        let t99 = t7 * t7;
        let t100 = 1.0 / t99;
        let t101 = t17 * t100;
        let t103 = piecewise5(t11, 0.0, t15, 0.0, t8 - t101);
        let t106 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t103);
        let t107 = t106 * t30;
        let t111 = 1.0 / t29;
        let t112 = t28 * t111;
        let t115 = t6 * t112 * t59 / 10.0;
        let t116 = t38 * rho0;
        let t118 = 1.0 / t40 / t116;
        let t123 = t46 * t46;
        let t124 = t37 * t123;
        let t125 = t50 * t116;
        let t127 = 1.0 / t40 / t125;
        let t128 = t54 * t54;
        let t129 = 1.0 / t128;
        let t134 = -5.0 / 243.0 * t37 * sigma0 * t118 * t55 + 5.0 / 186624.0 * t124 * t127 * t129 * t45;
        let t139 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t107 * t59 + t115 + 3.0 / 20.0 * t6 * t31 * t134);
        let t140 = t65 * t100;
        let t142 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t140);
        let t145 = piecewise3(t69, 0.0, 5.0 / 3.0 * t71 * t142);
        let t146 = t145 * t30;
        let t150 = t73 * t111;
        let t153 = t6 * t150 * t94 / 10.0;
        let t155 = piecewise3(t64, 0.0, 3.0 / 20.0 * t6 * t146 * t94 + t153);
        let tvrho0 = t63 + t98 + t7 * (t139 + t155);
        vrho[ip * 2] += tvrho0;
        let t159 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t101);
        let t162 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t159);
        let t163 = t162 * t30;
        let t168 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t163 * t59 + t115);
        let t170 = piecewise5(t15, 0.0, t11, 0.0, t8 - t140);
        let t173 = piecewise3(t69, 0.0, 5.0 / 3.0 * t71 * t170);
        let t174 = t173 * t30;
        let t178 = t75 * rho1;
        let t180 = 1.0 / t77 / t178;
        let t185 = t81 * t81;
        let t186 = t37 * t185;
        let t187 = t85 * t178;
        let t189 = 1.0 / t77 / t187;
        let t190 = t89 * t89;
        let t191 = 1.0 / t190;
        let t196 = -5.0 / 243.0 * t37 * sigma2 * t180 * t90 + 5.0 / 186624.0 * t186 * t189 * t191 * t45;
        let t201 = piecewise3(t64, 0.0, 3.0 / 20.0 * t6 * t174 * t94 + t153 + 3.0 / 20.0 * t6 * t74 * t196);
        let tvrho1 = t63 + t98 + t7 * (t168 + t201);
        vrho[ip * 2 + 1] += tvrho1;
        let t208 = t50 * t38;
        let t210 = 1.0 / t40 / t208;
        let t212 = t210 * t129 * t45;
        let t215 = 5.0 / 648.0 * t37 * t42 * t55 - 5.0 / 497664.0 * t37 * t47 * t212;
        let t219 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t215);
        let tvsigma0 = t7 * t219;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t224 = t85 * t75;
        let t226 = 1.0 / t77 / t224;
        let t228 = t226 * t191 * t45;
        let t231 = 5.0 / 648.0 * t37 * t79 * t90 - 5.0 / 497664.0 * t37 * t82 * t228;
        let t235 = piecewise3(t64, 0.0, 3.0 / 20.0 * t6 * t74 * t231);
        let tvsigma2 = t7 * t235;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
