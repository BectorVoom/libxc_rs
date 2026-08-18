//! LDA_X_ERF fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_erf_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t1 * t3 * t6;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = piecewise3(t10, t11 * zeta_threshold, 1.0);
        let t14 = t9 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = t17 * t18 * param_hyb_omega_0;
        let t23 = piecewise3(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t20 * t1 / t15 * t24 / 18.0;
        let t28 = 1.35 <= t27;
        let t29 = 1.35 < t27;
        let t30 = piecewise3(t29, t27, 1.35);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t44 = 1.0 / t40 / t31;
        let t47 = 1.0 / t40 / t34;
        let t50 = 1.0 / t40 / t37;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = piecewise3(t29, 1.35, t27);
        let t57 = f64::sqrt(M_PI);
        let t58 = 1.0 / t56;
        let t60 = erf_approx(t58 / 2.0);
        let t62 = t56 * t56;
        let t63 = 1.0 / t62;
        let t65 = f64::exp(-t63 / 4.0);
        let t66 = t65 - 1.0;
        let t69 = t65 - 3.0 / 2.0 - 2.0 * t62 * t66;
        let t72 = 2.0 * t56 * t69 + t57 * t60;
        let t76 = piecewise3(t28, 1.0 / t31 / 36.0 - t35 / 960.0 + t38 / 26880.0 - t41 / 829440.0 + t44 / 28385280.0 - t47 / 1073479680.0 + t50 / 44590694400.0 - t53 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t56 * t72);
        let t79 = t7 * t14 * t15 * t76;
        let tzk0 = -3.0 / 16.0 * t79;
        zk[ip] += tzk0;
        let t82 = t15 * rho[ip];
        let t84 = t82 * t1 * t3;
        let t85 = t6 * t9;
        let t86 = t31 * t30;
        let t87 = 1.0 / t86;
        let t92 = t20 * t1 / t82 * t24 / 54.0;
        let t93 = piecewise3(t29, -t92, 0.0);
        let t96 = t34 * t30;
        let t97 = 1.0 / t96;
        let t100 = t34 * t86;
        let t101 = 1.0 / t100;
        let t105 = 1.0 / t40 / t30;
        let t109 = 1.0 / t40 / t86;
        let t113 = 1.0 / t40 / t96;
        let t117 = 1.0 / t40 / t100;
        let t121 = 1.0 / t52 / t30;
        let t125 = piecewise3(t29, 0.0, -t92);
        let t127 = t65 * t63;
        let t131 = t62 * t56;
        let t132 = 1.0 / t131;
        let t136 = t56 * t66;
        let t141 = t132 * t125 * t65 / 2.0 - 4.0 * t136 * t125 - t58 * t125 * t65;
        let t144 = -t127 * t125 + 2.0 * t125 * t69 + 2.0 * t56 * t141;
        let t148 = piecewise3(t28, -t87 * t93 / 18.0 + t97 * t93 / 240.0 - t101 * t93 / 4480.0 + t105 * t93 / 103680.0 - t109 * t93 / 2838528.0 + t113 * t93 / 89456640.0 - t117 * t93 / 3185049600.0 + t121 * t93 / 126340300800.0, -8.0 / 3.0 * t125 * t72 - 8.0 / 3.0 * t56 * t144);
        let tvrho0 = -t79 / 4.0 - 3.0 / 16.0 * t84 * t85 * t13 * t148;
        vrho[ip] += tvrho0;
        let t153 = t15 * t15;
        let t154 = 1.0 / t153;
        let t163 = t93 * t93;
        let t166 = rho[ip] * rho[ip];
        let t172 = 2.0 / 81.0 * t20 * t1 / t15 / t166 * t24;
        let t173 = piecewise3(t29, t172, 0.0);
        let t201 = 1.0 / t52 / t31;
        let t206 = t35 * t163 / 6.0 - t87 * t173 / 18.0 - t38 * t163 / 48.0 + t97 * t173 / 240.0 + t41 * t163 / 640.0 - t101 * t173 / 4480.0 - t44 * t163 / 11520.0 + t105 * t173 / 103680.0 + t47 * t163 / 258048.0 - t109 * t173 / 2838528.0 - t50 * t163 / 6881280.0 + t113 * t173 / 89456640.0 + t53 * t163 / 212336640.0 - t117 * t173 / 3185049600.0 - t201 * t163 / 7431782400.0 + t121 * t173 / 126340300800.0;
        let t207 = piecewise3(t29, 0.0, t172);
        let t212 = t62 * t62;
        let t214 = 1.0 / t212 / t56;
        let t215 = t125 * t125;
        let t216 = t214 * t215;
        let t219 = t65 * t132;
        let t227 = 1.0 / t212;
        let t235 = 1.0 / t212 / t62;
        let t236 = t235 * t215;
        let t247 = -2.0 * t227 * t215 * t65 + t132 * t207 * t65 / 2.0 + t236 * t65 / 4.0 - 4.0 * t215 * t66 - t63 * t215 * t65 - 4.0 * t136 * t207 - t58 * t207 * t65;
        let t250 = -t216 * t65 / 2.0 + 2.0 * t219 * t215 - t127 * t207 + 2.0 * t207 * t69 + 4.0 * t125 * t141 + 2.0 * t56 * t247;
        let t254 = piecewise3(t28, t206, -8.0 / 3.0 * t207 * t72 - 16.0 / 3.0 * t125 * t144 - 8.0 / 3.0 * t56 * t250);
        let tv2rho20 = -t7 * t14 * t154 * t76 / 12.0 - t7 * t14 * t15 * t148 / 2.0 - 3.0 / 16.0 * t84 * t85 * t13 * t254;
        v2rho2[ip] += tv2rho20;
    }
}
