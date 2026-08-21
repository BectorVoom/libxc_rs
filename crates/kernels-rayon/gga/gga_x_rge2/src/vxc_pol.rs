//! GGA_X_RGE2 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_rge2_vxc_pol(
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
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t33 = t28 / t31;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t42 = t28 * t28;
        let t44 = 1.0 / t30 / t29;
        let t45 = t42 * t44;
        let t46 = sigma0 * sigma0;
        let t47 = t34 * t34;
        let t48 = t47 * rho0;
        let t50 = 1.0 / t35 / t48;
        let t54 = 0.804 + 5.0 / 972.0 * t33 * sigma0 * t38 + 3.291178445357254e-05 * t45 * t46 * t50;
        let t57 = 1.804 - 0.646416 / t54;
        let t61 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t16;
        let t65 = piecewise5(t14, t11, t10, t15, t63 * t7);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3(t66);
        let t70 = piecewise3(t67, t22, t68 * t66);
        let t72 = rho1 * rho1;
        let t73 = pow_1_3(rho1);
        let t74 = t73 * t73;
        let t76 = 1.0 / t74 / t72;
        let t80 = sigma2 * sigma2;
        let t81 = t72 * t72;
        let t82 = t81 * rho1;
        let t84 = 1.0 / t73 / t82;
        let t88 = 0.804 + 5.0 / 972.0 * t33 * sigma2 * t76 + 3.291178445357254e-05 * t45 * t80 * t84;
        let t91 = 1.804 - 0.646416 / t88;
        let t95 = piecewise3(t62, 0.0, -3.0 / 8.0 * t5 * t70 * t26 * t91);
        let tzk0 = t61 + t95;
        zk[ip] += tzk0;
        let t96 = t6 * t6;
        let t97 = 1.0 / t96;
        let t98 = t16 * t97;
        let t100 = piecewise5(t10, 0.0, t14, 0.0, t7 - t98);
        let t103 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t100);
        let t108 = t26 * t26;
        let t109 = 1.0 / t108;
        let t113 = t5 * t25 * t109 * t57 / 8.0;
        let t114 = t2 * t25;
        let t115 = t54 * t54;
        let t116 = 1.0 / t115;
        let t117 = t26 * t116;
        let t118 = t34 * rho0;
        let t120 = 1.0 / t36 / t118;
        let t124 = t47 * t34;
        let t126 = 1.0 / t35 / t124;
        let t130 = -10.0 / 729.0 * t33 * sigma0 * t120 - 0.00017552951708572022 * t45 * t46 * t126;
        let t131 = t117 * t130;
        let t135 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t103 * t26 * t57 - t113 - 0.1655109536374632 * t114 * t131);
        let t136 = t63 * t97;
        let t138 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t136);
        let t141 = piecewise3(t67, 0.0, 4.0 / 3.0 * t68 * t138);
        let t149 = t5 * t70 * t109 * t91 / 8.0;
        let t151 = piecewise3(t62, 0.0, -3.0 / 8.0 * t5 * t141 * t26 * t91 - t149);
        let tvrho0 = t61 + t95 + t6 * (t135 + t151);
        vrho[ip * 2] += tvrho0;
        let t155 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t98);
        let t158 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t155);
        let t164 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t158 * t26 * t57 - t113);
        let t166 = piecewise5(t14, 0.0, t10, 0.0, t7 - t136);
        let t169 = piecewise3(t67, 0.0, 4.0 / 3.0 * t68 * t166);
        let t174 = t2 * t70;
        let t175 = t88 * t88;
        let t176 = 1.0 / t175;
        let t177 = t26 * t176;
        let t178 = t72 * rho1;
        let t180 = 1.0 / t74 / t178;
        let t184 = t81 * t72;
        let t186 = 1.0 / t73 / t184;
        let t190 = -10.0 / 729.0 * t33 * sigma2 * t180 - 0.00017552951708572022 * t45 * t80 * t186;
        let t191 = t177 * t190;
        let t195 = piecewise3(t62, 0.0, -3.0 / 8.0 * t5 * t169 * t26 * t91 - t149 - 0.1655109536374632 * t174 * t191);
        let tvrho1 = t61 + t95 + t6 * (t164 + t195);
        vrho[ip * 2 + 1] += tvrho1;
        let t203 = 5.0 / 972.0 * t33 * t38 + 6.582356890714508e-05 * t45 * sigma0 * t50;
        let t204 = t117 * t203;
        let t207 = piecewise3(t1, 0.0, -0.1655109536374632 * t114 * t204);
        let tvsigma0 = t6 * t207;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t213 = 5.0 / 972.0 * t33 * t76 + 6.582356890714508e-05 * t45 * sigma2 * t84;
        let t214 = t177 * t213;
        let t217 = piecewise3(t62, 0.0, -0.1655109536374632 * t174 * t214);
        let tvsigma2 = t6 * t217;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
