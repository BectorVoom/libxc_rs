//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1944/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1944<F: Float>(t114: F, t101760: F, t2327: F, t7968: F, t26179: F, t28133: F, t7706: F, t95293: F, t60224: F, t7342: F, t13272: F, t26178: F, t6960: F) -> (F, F, F, F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t101761 = piecewise3::<F>(t115, F::new(0.0), t101760);
    let t101767 = t7968 * t2327;
    let t101782 = F::new(80.0) / F::new(9.0) * t26179 * t28133;
    let t101783 = t95293 * t7706;
    let t101785 = t60224 * t7342;
    let t101788 = t13272 * t26178;
    let t101790 = F::new(80.0) / F::new(9.0) * t101788 * t6960;
    (t101761, t101767, t101782, t101783, t101785, t101790)
}
