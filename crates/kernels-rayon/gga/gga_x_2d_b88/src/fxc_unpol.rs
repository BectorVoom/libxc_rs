//! GGA_X_2D_B88 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b88.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b88_fxc_unpol(
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
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = rmath::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = rmath::sqrt(zeta_threshold);
        let t13 = rmath::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = rmath::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t24 = rmath::sqrt(sigma[ip]);
        let t25 = t24 * t17;
        let t27 = 1.0 / t18 / rho[ip];
        let t29 = rmath::ln(t25 * t27 + rmath::sqrt(pow_2(t25 * t27) + 1.0));
        let t30 = t27 * t29;
        let t33 = 1.0 + 0.056 * t25 * t30;
        let t34 = 1.0 / t33;
        let t37 = 1.0 + 0.009305382717253959 * t23 * t34;
        let t41 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t43 = t17 / t18;
        let t47 = t20 * t20;
        let t48 = 1.0 / t47;
        let t49 = sigma[ip] * t48;
        let t52 = t33 * t33;
        let t53 = 1.0 / t52;
        let t55 = 1.0 / t18 / t20;
        let t56 = t55 * t29;
        let t60 = 2.0 * t23 + 1.0;
        let t61 = rmath::sqrt(t60);
        let t62 = 1.0 / t61;
        let t65 = -0.084 * t25 * t56 - 0.168 * t49 * t62;
        let t66 = t53 * t65;
        let t69 = -0.02791614815176188 * t49 * t34 - 0.009305382717253959 * t23 * t66;
        let t74 = piecewise3(t2, 0.0, -t16 * t43 * t37 / 3.0 - 2.0 / 3.0 * t16 * t19 * t69);
        let tvrho0 = 2.0 * rho[ip] * t74 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t80 = 1.0 / t24 * t17;
        let t85 = 0.028 * t80 * t30 + 0.056 * t22 * t62;
        let t86 = t53 * t85;
        let t89 = 0.009305382717253959 * t22 * t34 - 0.009305382717253959 * t23 * t86;
        let t93 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
        let t96 = t17 * t27;
        let t103 = t47 * rho[ip];
        let t104 = 1.0 / t103;
        let t105 = sigma[ip] * t104;
        let t111 = 1.0 / t52 / t33;
        let t112 = t65 * t65;
        let t113 = t111 * t112;
        let t117 = 1.0 / t18 / t21;
        let t118 = t117 * t29;
        let t123 = sigma[ip] * sigma[ip];
        let t124 = t47 * t47;
        let t125 = 1.0 / t124;
        let t128 = 1.0 / t61 / t60;
        let t131 = 0.21 * t25 * t118 + 0.924 * t105 * t62 - 0.504 * t123 * t125 * t128;
        let t132 = t53 * t131;
        let t135 = 0.11166459260704752 * t105 * t34 + 0.05583229630352376 * t49 * t66 + 0.018610765434507917 * t23 * t113 - 0.009305382717253959 * t23 * t132;
        let t140 = piecewise3(t2, 0.0, t16 * t96 * t37 / 6.0 - 2.0 / 3.0 * t16 * t43 * t69 - 2.0 / 3.0 * t16 * t19 * t135);
        let tv2rho20 = 2.0 * rho[ip] * t140 + 4.0 * t74;
        v2rho2[ip] += tv2rho20;
        let t148 = t22 * t53;
        let t153 = t111 * t85;
        let t154 = t153 * t65;
        let t161 = t47 * t21;
        let t162 = 1.0 / t161;
        let t163 = t162 * t128;
        let t166 = -0.042 * t80 * t56 - 0.252 * t48 * t62 + 0.168 * t163 * sigma[ip];
        let t167 = t53 * t166;
        let t170 = -0.02791614815176188 * t48 * t34 - 0.009305382717253959 * t148 * t65 + 0.02791614815176188 * t49 * t86 + 0.018610765434507917 * t23 * t154 - 0.009305382717253959 * t23 * t167;
        let t175 = piecewise3(t2, 0.0, -t16 * t43 * t89 / 3.0 - 2.0 / 3.0 * t16 * t19 * t170);
        let tv2rhosigma0 = 2.0 * rho[ip] * t175 + 2.0 * t93;
        v2rhosigma[ip] += tv2rhosigma0;
        let t180 = t85 * t85;
        let t181 = t111 * t180;
        let t186 = 1.0 / t24 / sigma[ip] * t17;
        let t189 = 1.0 / sigma[ip];
        let t193 = t47 * t20;
        let t194 = 1.0 / t193;
        let t197 = -0.014 * t186 * t30 + 0.028 * t189 * t22 * t62 - 0.056 * t194 * t128;
        let t198 = t53 * t197;
        let t201 = -0.018610765434507917 * t148 * t85 + 0.018610765434507917 * t23 * t181 - 0.009305382717253959 * t23 * t198;
        let t205 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t201);
        let tv2sigma20 = 2.0 * rho[ip] * t205;
        v2sigma2[ip] += tv2sigma20;
    }
}
