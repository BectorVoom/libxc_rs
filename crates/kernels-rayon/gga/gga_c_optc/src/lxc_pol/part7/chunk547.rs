//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 547/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk547(t2671: f64, t2679: f64, t2622: f64, t2623: f64, t2627: f64, t2630: f64, t2635: f64, t2640: f64, t2645: f64, t2650: f64, t2655: f64, t2659: f64, t2668: f64, t2675: f64, t2678: f64, t862: f64, t867: f64, t878: f64, t893: f64) -> (f64, f64) {
    let t2680 = t2671 * t2679;
    let t2683 = -t2622 - t2623 * t867 / 54.0_f64 + t2627 / 432.0_f64 + t862 * t2630 / 288.0_f64 + t862 * t2635 / 216.0_f64 + 0.47333755318775392234e-1_f64 * t2640 * t2645 - 0.36221506207453157728e-2_f64 * t893 * t2650 - 0.37867004255020313788e0_f64 * t2655 * t878 + 0.47333755318775392234e-1_f64 * t2659 + 0.9157278480459830169e1_f64 * t2668 * t2675 - 0.45786392402299150845e1_f64 * t2678 * t2680;
    (t2680, t2683)
}
