//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1293/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1293<F: Float>(t12459: F, t12460: F, t16710: F, t16931: F, t17066: F, t17075: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F, t12261: F, t12297: F, t16706: F, t16876: F, t17050: F, t17052: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t20498: F) -> F {
    let t20518 = -F::new(0.157790625e0) * t20366 + F::new(0.6311625e0) * t20368 + F::new(0.31558125e0) * t20371 - t17066 - t12459 - t12460 + F::new(0.6311625e0) * t20373 - F::new(0.68863333333333333332e0) * t16710 + t17075 + F::new(0.4630888888888888889e-1) * t16931 + F::new(0.46308888888888888889e-1) * t20378;
    let t20520 = F::new(0.11577222222222222222e0) * t12261 - t17050 - t17052 - F::new(0.34731666666666666667e-1) * t20268 + F::new(0.45908888888888888888e0) * t16706 + F::new(0.23154444444444444445e0) * t16876 + F::new(0.104195e0) * t20274 + F::new(0.23154444444444444445e-1) * t20276 - F::new(0.13892666666666666667e0) * t20278 - F::new(0.69463333333333333333e-1) * t20280 + t20498 + F::new(0.3529725e1) * t20338 + F::new(0.20839e0) * t20341 - F::new(0.69463333333333333334e-1) * t20344 - F::new(0.20839e0) * t20347 + F::new(0.41678e0) * t20350 + F::new(0.62517e0) * t20353 + F::new(0.22954444444444444444e0) * t12297 + F::new(0.264729375e1) * t20357 - F::new(0.3529725e1) * t20359 - F::new(0.17648625e1) * t20362 + t20518;
    t20520
}
