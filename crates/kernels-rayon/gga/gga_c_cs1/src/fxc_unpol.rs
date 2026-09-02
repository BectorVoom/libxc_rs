//! GGA_C_CS1 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_cs1_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.349 * t2;
        let t5 = 1.0 / t4;
        let t6 = sigma[ip] * sigma[ip];
        let t7 = rho[ip] * rho[ip];
        let t8 = t7 * t7;
        let t9 = t8 * rho[ip];
        let t11 = 1.0 / t1 / t9;
        let t13 = t1 * t1;
        let t15 = 1.0 / t13 / t7;
        let t18 = 1.0 + 0.006 * sigma[ip] * t15;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t23 = -0.159068 + 2.86308e-07 * t6 * t11 * t20;
        let t25 = t5 * t23 / 4.0;
        let t27 = piecewise3(1.0 <= zeta_threshold, zeta_threshold, 1.0);
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = t27 * t29;
        let t33 = t29 * t1 / 2.0 + 0.349;
        let t34 = 1.0 / t33;
        let t35 = t1 * t34;
        let t36 = t6 * t28;
        let t37 = sigma[ip] * t29;
        let t40 = 1.0 + 0.006 * t37 * t15;
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t11 * t42;
        let t46 = -0.018897 + 1.117728e-05 * t36 * t43;
        let t49 = t30 * t35 * t46 / 2.0;
        let tzk0 = t25 + t49;
        zk[ip] += tzk0;
        let t50 = t4 * t4;
        let t51 = 1.0 / t50;
        let t52 = t51 * t23;
        let t54 = 1.0 / t1 / rho[ip];
        let t55 = t52 * t54;
        let t57 = t8 * t7;
        let t59 = 1.0 / t1 / t57;
        let t63 = t6 * sigma[ip];
        let t64 = t8 * t8;
        let t65 = t64 * rho[ip];
        let t66 = 1.0 / t65;
        let t67 = t63 * t66;
        let t69 = 1.0 / t19 / t18;
        let t72 = -1.526976e-06 * t6 * t59 * t20 + 9.161856e-09 * t67 * t69;
        let t73 = t5 * t72;
        let t76 = 1.0 / t13 * t34;
        let t78 = t30 * t76 * t46;
        let t80 = t27 * t28;
        let t81 = t33 * t33;
        let t82 = 1.0 / t81;
        let t83 = t2 * t82;
        let t85 = t80 * t83 * t46;
        let t87 = t59 * t42;
        let t91 = 1.0 / t41 / t40;
        let t94 = -5.961216e-05 * t36 * t87 + 7.1534592e-07 * t67 * t91;
        let t96 = t30 * t35 * t94;
        let tvrho0 = t25 + t49 + rho[ip] * (0.029083333333333333 * t55 + t73 / 4.0 + t78 / 6.0 - t85 / 6.0 + t96 / 2.0);
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t64;
        let t104 = t6 * t103;
        let t107 = 5.72616e-07 * sigma[ip] * t11 * t20 - 3.435696e-09 * t104 * t69;
        let t109 = t5 * t107 / 4.0;
        let t110 = sigma[ip] * t28;
        let t115 = 2.235456e-05 * t110 * t43 - 2.6825472e-07 * t104 * t91;
        let t118 = t30 * t35 * t115 / 2.0;
        let tvsigma0 = rho[ip] * (t109 + t118);
        vsigma[ip] += tvsigma0;
        let t125 = 1.0 / t50 / t4;
        let t126 = t125 * t23;
        let t127 = t126 * t15;
        let t129 = t51 * t72;
        let t130 = t129 * t54;
        let t133 = 1.0 / t1 / t7;
        let t134 = t52 * t133;
        let t136 = t7 * rho[ip];
        let t137 = t8 * t136;
        let t139 = 1.0 / t1 / t137;
        let t143 = t64 * t7;
        let t144 = 1.0 / t143;
        let t145 = t63 * t144;
        let t148 = t6 * t6;
        let t149 = t64 * t8;
        let t151 = 1.0 / t13 / t149;
        let t152 = t148 * t151;
        let t153 = t19 * t19;
        let t154 = 1.0 / t153;
        let t157 = 9.670848e-06 * t6 * t139 * t20 - 1.31319936e-07 * t145 * t69 + 4.39769088e-10 * t152 * t154;
        let t158 = t5 * t157;
        let t161 = 1.0 / t13 / rho[ip];
        let t162 = t161 * t34;
        let t164 = t30 * t162 * t46;
        let t167 = t30 * t76 * t94;
        let t170 = t27 / rho[ip];
        let t172 = 1.0 / t81 / t33;
        let t173 = t172 * t46;
        let t174 = t170 * t173;
        let t177 = t80 * t83 * t94;
        let t179 = t139 * t42;
        let t184 = t41 * t41;
        let t185 = 1.0 / t184;
        let t186 = t185 * t29;
        let t189 = 0.00037754368 * t36 * t179 - 1.025329152e-05 * t145 * t91 + 3.433660416e-08 * t152 * t186;
        let t191 = t30 * t35 * t189;
        let tv2rho20 = 0.058166666666666665 * t55 + t73 / 2.0 + t78 / 3.0 - t85 / 3.0 + t96 + rho[ip] * (0.006766722222222223 * t127 + 0.058166666666666665 * t130 - 0.03877777777777778 * t134 + t158 / 4.0 - t164 / 9.0 + t167 / 3.0 + t174 / 9.0 - t177 / 3.0 + t191 / 2.0);
        v2rho2[ip] += tv2rho20;
        let t195 = t51 * t107;
        let t196 = t195 * t54;
        let t201 = t6 * t66;
        let t204 = t64 * t136;
        let t206 = 1.0 / t13 / t204;
        let t207 = t63 * t206;
        let t210 = -3.053952e-06 * sigma[ip] * t59 * t20 + 4.580928e-08 * t201 * t69 - 1.64913408e-10 * t207 * t154;
        let t211 = t5 * t210;
        let t214 = t30 * t76 * t115;
        let t217 = t80 * t83 * t115;
        let t225 = -0.00011922432 * t110 * t87 + 3.5767296e-06 * t201 * t91 - 1.287622656e-08 * t207 * t186;
        let t227 = t30 * t35 * t225;
        let tv2rhosigma0 = t109 + t118 + rho[ip] * (0.029083333333333333 * t196 + t211 / 4.0 + t214 / 6.0 - t217 / 6.0 + t227 / 2.0);
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = sigma[ip] * t103;
        let t237 = 1.0 / t13 / t143;
        let t238 = t6 * t237;
        let t241 = 5.72616e-07 * t11 * t20 - 1.3742784e-08 * t233 * t69 + 6.1842528e-11 * t238 * t154;
        let t243 = t5 * t241 / 4.0;
        let t251 = 2.235456e-05 * t28 * t11 * t42 - 1.07301888e-06 * t233 * t91 + 4.82858496e-09 * t238 * t186;
        let t254 = t30 * t35 * t251 / 2.0;
        let tv2sigma20 = rho[ip] * (t243 + t254);
        v2sigma2[ip] += tv2sigma20;
    }
}
