//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1619/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1619<F: Float>(t12459: F, t12460: F, t16710: F, t16931: F, t17066: F, t17075: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F, t12261: F, t12297: F, t16706: F, t16876: F, t17050: F, t17052: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t20498: F) -> F {
    let t20518 = -F::cast_from(0.157790625e0_f64) * t20366 + F::new(0.6311625e0) * t20368 + F::new(0.31558125e0) * t20371 - t17066 - t12459 - t12460 + F::new(0.6311625e0) * t20373 - F::cast_from(0.68863333333333333332e0_f64) * t16710 + t17075 + F::cast_from(0.4630888888888888889e-1_f64) * t16931 + F::cast_from(0.46308888888888888889e-1_f64) * t20378;
    let t20520 = F::cast_from(0.11577222222222222222e0_f64) * t12261 - t17050 - t17052 - F::cast_from(0.34731666666666666667e-1_f64) * t20268 + F::cast_from(0.45908888888888888888e0_f64) * t16706 + F::cast_from(0.23154444444444444445e0_f64) * t16876 + F::new(0.104195e0) * t20274 + F::cast_from(0.23154444444444444445e-1_f64) * t20276 - F::cast_from(0.13892666666666666667e0_f64) * t20278 - F::cast_from(0.69463333333333333333e-1_f64) * t20280 + t20498 + F::new(0.3529725e1) * t20338 + F::new(0.20839e0) * t20341 - F::cast_from(0.69463333333333333334e-1_f64) * t20344 - F::new(0.20839e0) * t20347 + F::new(0.41678e0) * t20350 + F::new(0.62517e0) * t20353 + F::cast_from(0.22954444444444444444e0_f64) * t12297 + F::cast_from(0.264729375e1_f64) * t20357 - F::new(0.3529725e1) * t20359 - F::new(0.17648625e1) * t20362 + t20518;
    t20520
}
