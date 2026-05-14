//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 537/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk537<F: Float>(t2671: F, t2679: F, t2622: F, t2623: F, t2627: F, t2630: F, t2635: F, t2640: F, t2645: F, t2650: F, t2655: F, t2659: F, t2668: F, t2675: F, t2678: F, t862: F, t867: F, t878: F, t893: F) -> (F, F) {
    let t2680 = t2671 * t2679;
    let t2683 = -t2622 - t2623 * t867 / 54.0 + t2627 / 432.0 + t862 * t2630 / 288.0 + t862 * t2635 / 216.0 + 0.47333755318775392234e-1 * t2640 * t2645 - 0.36221506207453157728e-2 * t893 * t2650 - 0.37867004255020313788e0 * t2655 * t878 + 0.47333755318775392234e-1 * t2659 + 0.9157278480459830169e1 * t2668 * t2675 - 0.45786392402299150845e1 * t2678 * t2680;
    (t2680, t2683)
}
