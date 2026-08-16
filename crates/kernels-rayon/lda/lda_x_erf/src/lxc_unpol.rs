//! LDA_X_ERF lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::erf::{erf_approx};

/// LDA_X_ERF lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_erf_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
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
        let t260 = 1.0 / t153 / rho[ip];
        let t273 = t163 * t93;
        let t276 = t35 * t93;
        let t281 = t38 * t93;
        let t286 = t41 * t93;
        let t291 = t44 * t93;
        let t296 = t47 * t93;
        let t301 = t50 * t93;
        let t304 = -2.0 / 3.0 * t97 * t273 + t276 * t173 / 2.0 + t101 * t273 / 8.0 - t281 * t173 / 16.0 - t105 * t273 / 80.0 + 3.0 / 640.0 * t286 * t173 + t109 * t273 / 1152.0 - t291 * t173 / 3840.0 - t113 * t273 / 21504.0 + t296 * t173 / 86016.0 + t117 * t273 / 491520.0 - t301 * t173 / 2293760.0;
        let t307 = t53 * t93;
        let t311 = 1.0 / t52 / t86;
        let t314 = t201 * t93;
        let t323 = 14.0 / 243.0 * t20 * t1 / t15 / t166 / rho[ip] * t24;
        let t324 = piecewise3(t29, -t323, 0.0);
        let t341 = -t121 * t273 / 13271040.0 + t307 * t173 / 70778880.0 + t311 * t273 / 412876800.0 - t314 * t173 / 2477260800.0 - t87 * t324 / 18.0 + t97 * t324 / 240.0 - t101 * t324 / 4480.0 + t105 * t324 / 103680.0 - t109 * t324 / 2838528.0 + t113 * t324 / 89456640.0 - t117 * t324 / 3185049600.0 + t121 * t324 / 126340300800.0;
        let t343 = piecewise3(t29, 0.0, -t323);
        let t350 = t215 * t125;
        let t354 = t214 * t125;
        let t355 = t65 * t207;
        let t358 = t212 * t212;
        let t359 = 1.0 / t358;
        let t363 = t65 * t227;
        let t379 = t227 * t125;
        let t383 = 1.0 / t212 / t131;
        let t391 = t125 * t65;
        let t395 = 1.0 / t358 / t56;
        let t399 = t125 * t66;
        let t402 = t63 * t125;
        let t409 = 15.0 / 2.0 * t214 * t350 * t65 - 6.0 * t379 * t355 - 5.0 / 2.0 * t383 * t350 * t65 + t132 * t343 * t65 / 2.0 + 3.0 / 4.0 * t235 * t207 * t391 + t395 * t350 * t65 / 8.0 - 12.0 * t399 * t207 - 3.0 * t402 * t355 - 4.0 * t136 * t343 - t58 * t343 * t65;
        let t412 = 7.0 / 2.0 * t235 * t350 * t65 - 3.0 / 2.0 * t354 * t355 - t359 * t350 * t65 / 4.0 - 6.0 * t363 * t350 + 6.0 * t219 * t125 * t207 - t127 * t343 + 2.0 * t343 * t69 + 6.0 * t207 * t141 + 6.0 * t125 * t247 + 2.0 * t56 * t409;
        let t416 = piecewise3(t28, t304 + t341, -8.0 / 3.0 * t343 * t72 - 8.0 * t207 * t144 - 8.0 * t125 * t250 - 8.0 / 3.0 * t56 * t412);
        let tv3rho30 = t7 * t14 * t260 * t76 / 18.0 - t7 * t14 * t154 * t148 / 4.0 - 3.0 / 4.0 * t7 * t14 * t15 * t254 - 3.0 / 16.0 * t84 * t85 * t13 * t416;
        v3rho3[ip] += tv3rho30;
        let t438 = t166 * t166;
        let t444 = 140.0 / 729.0 * t20 * t1 / t15 / t438 * t24;
        let t445 = piecewise3(t29, t444, 0.0);
        let t462 = t163 * t163;
        let t465 = t173 * t173;
        let t488 = -t87 * t445 / 18.0 + t97 * t445 / 240.0 - t101 * t445 / 4480.0 + t105 * t445 / 103680.0 - t109 * t445 / 2838528.0 + t113 * t445 / 89456640.0 - t117 * t445 / 3185049600.0 + t121 * t445 / 126340300800.0 + 10.0 / 3.0 * t38 * t462 + t35 * t465 / 2.0 - 7.0 / 8.0 * t41 * t462 - t38 * t465 / 16.0 + 9.0 / 80.0 * t44 * t462 + 3.0 / 640.0 * t41 * t465 - 11.0 / 1152.0 * t47 * t462 - t44 * t465 / 3840.0 + 13.0 / 21504.0 * t50 * t462 + t47 * t465 / 86016.0 - t53 * t462 / 32768.0 - t50 * t465 / 2293760.0;
        let t539 = 17.0 / 13271040.0 * t201 * t462 + t53 * t465 / 70778880.0 - 19.0 / 412876800.0 / t52 / t34 * t462 - t201 * t465 / 2477260800.0 + t286 * t324 / 160.0 + t109 * t163 * t173 / 192.0 - t291 * t324 / 2880.0 - t113 * t163 * t173 / 3584.0 + t296 * t324 / 64512.0 + t117 * t163 * t173 / 81920.0 - t301 * t324 / 1720320.0 - t121 * t163 * t173 / 2211840.0 + t307 * t324 / 53084160.0 + t311 * t163 * t173 / 68812800.0 - t314 * t324 / 1857945600.0 - 4.0 * t97 * t163 * t173 + 2.0 / 3.0 * t276 * t324 + 3.0 / 4.0 * t101 * t163 * t173 - t281 * t324 / 12.0 - 3.0 / 40.0 * t105 * t163 * t173;
        let t541 = piecewise3(t29, 0.0, t444);
        let t550 = t215 * t215;
        let t565 = t207 * t207;
        let t585 = t65 * t343;
        let t613 = 1.0 / t358 / t212 * t550 * t65 / 16.0 - 12.0 * t565 * t66 - 16.0 * t399 * t343 - 4.0 * t136 * t541 - t58 * t541 * t65 + 85.0 / 4.0 * t359 * t550 * t65 - 19.0 / 8.0 / t358 / t62 * t550 * t65 + t132 * t541 * t65 / 2.0 - 8.0 * t379 * t585 - 15.0 * t383 * t215 * t355 + t235 * t343 * t391 + 3.0 / 4.0 * t235 * t565 * t65 + 3.0 / 4.0 * t395 * t207 * t215 * t65 - 3.0 * t63 * t565 * t65 - 4.0 * t402 * t585 - 75.0 / 2.0 * t235 * t550 * t65 + 45.0 * t216 * t355 - 6.0 * t227 * t565 * t65;
        let t648 = 15.0 / 4.0 * t395 * t550 * t65 - 1.0 / t358 / t131 * t550 * t65 / 8.0 - t127 * t541 + 2.0 * t56 * t613 + 2.0 * t541 * t69 + 8.0 * t343 * t141 + 12.0 * t207 * t247 + 8.0 * t125 * t409 + 24.0 * t65 * t214 * t550 - 36.0 * t363 * t215 * t207 + 6.0 * t219 * t565 + 8.0 * t219 * t125 * t343 - 24.0 * t383 * t550 * t65 + 21.0 * t236 * t355 - 3.0 / 2.0 * t214 * t565 * t65 - 2.0 * t354 * t585 - 3.0 / 2.0 * t359 * t215 * t355;
        let t652 = piecewise3(t28, t488 + t539, -8.0 / 3.0 * t541 * t72 - 32.0 / 3.0 * t343 * t144 - 16.0 * t207 * t250 - 32.0 / 3.0 * t125 * t412 - 8.0 / 3.0 * t56 * t648);
        let tv4rho40 = -5.0 / 54.0 * t7 * t14 / t153 / t166 * t76 + 2.0 / 9.0 * t7 * t14 * t260 * t148 - t7 * t14 * t154 * t254 / 2.0 - t7 * t14 * t15 * t416 - 3.0 / 16.0 * t84 * t85 * t13 * t652;
        v4rho4[ip] += tv4rho40;
    }
}
