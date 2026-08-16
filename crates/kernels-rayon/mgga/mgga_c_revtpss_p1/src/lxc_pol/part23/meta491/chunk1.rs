//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1967/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1967(t12459: f64, t12460: f64, t16710: f64, t16931: f64, t17066: f64, t17075: f64, t20366: f64, t20368: f64, t20371: f64, t20373: f64, t20378: f64, t12261: f64, t12297: f64, t16706: f64, t16876: f64, t17050: f64, t17052: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64, t20357: f64, t20359: f64, t20362: f64, t20498: f64) -> f64 {
    let t20518 = -0.157790625e0_f64 * t20366 + 0.6311625e0_f64 * t20368 + 0.31558125e0_f64 * t20371 - t17066 - t12459 - t12460 + 0.6311625e0_f64 * t20373 - 0.68863333333333333332e0_f64 * t16710 + t17075 + 0.4630888888888888889e-1_f64 * t16931 + 0.46308888888888888889e-1_f64 * t20378;
    let t20520 = 0.11577222222222222222e0_f64 * t12261 - t17050 - t17052 - 0.34731666666666666667e-1_f64 * t20268 + 0.45908888888888888888e0_f64 * t16706 + 0.23154444444444444445e0_f64 * t16876 + 0.104195e0_f64 * t20274 + 0.23154444444444444445e-1_f64 * t20276 - 0.13892666666666666667e0_f64 * t20278 - 0.69463333333333333333e-1_f64 * t20280 + t20498 + 0.3529725e1_f64 * t20338 + 0.20839e0_f64 * t20341 - 0.69463333333333333334e-1_f64 * t20344 - 0.20839e0_f64 * t20347 + 0.41678e0_f64 * t20350 + 0.62517e0_f64 * t20353 + 0.22954444444444444444e0_f64 * t12297 + 0.264729375e1_f64 * t20357 - 0.3529725e1_f64 * t20359 - 0.17648625e1_f64 * t20362 + t20518;
    t20520
}
