//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 810/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk810<F: Float>(t7604: F, t828: F, t837: F, t845: F, t7523: F, t7525: F, t7527: F, t7529: F, t7531: F, t7535: F, t7538: F, t7541: F, t7544: F, t7547: F, t7550: F) -> (F, F, F) {
    let t7606 = t828 * t7604 * t837;
    let t7608 = F::new(0.58482233974552040708e0) * t845 * t7606;
    let t7609 = F::new(0.28842592592592592592e-1) * t7523;
    let t7620 = -t7609 - F::new(0.12361111111111111111e-1) * t7525 + F::new(0.61805555555555555556e-2) * t7527 - F::new(0.18541666666666666667e-1) * t7529 + F::new(0.92708333333333333334e-2) * t7531 - F::new(0.10300925925925925926e-1) * t7535 + F::new(0.37083333333333333333e-1) * t7538 - F::new(0.18541666666666666666e-1) * t7541 - F::new(0.55625000000000000001e-1) * t7544 + F::new(0.55625000000000000001e-1) * t7547 - F::new(0.92708333333333333333e-2) * t7550;
    (t7606, t7608, t7620)
}
