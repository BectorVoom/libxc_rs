//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk901<F: Float>(t8557: F, t8567: F, t8571: F, t8576: F, t8579: F, t8585: F, t8682: F, t8691: F, t8742: F, t8901: F, t8903: F, t1214: F, t2905: F, t8639: F, t8642: F, t8587: F, t8589: F, t8591: F, t8622: F, t8625: F, t8628: F, t8630: F, t8632: F, t8636: F) -> (F, F, F) {
    let t9265 = -t8742 + t8901 - t8567 + t8571 + t8576 + t8579 - t8585 + t8682 + t8691 + t8903 - t8557;
    let t9266 = t2905 * t1214;
    let t9268 = 0.60319259259259259259e1 * t8639;
    let t9269 = 0.54733333333333333333e-2 * t8642;
    let t9279 = -t9268 - t9269 - 0.21542592592592592592e1 * t8622 - 0.19388333333333333333e1 * t8625 - 0.4105e-2 * t8628 + 0.2463e-2 * t8630 + 0.821e-3 * t8632 - 0.54733333333333333333e-3 * t8636 - 0.12315e-2 * t8587 - 0.2585111111111111111e1 * t8589 + 0.19388333333333333333e1 * t8591;
    (t9265, t9266, t9279)
}
