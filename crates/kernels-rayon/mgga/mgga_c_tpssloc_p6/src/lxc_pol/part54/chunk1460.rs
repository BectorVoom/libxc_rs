//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1460/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1460(t113875: f64, t116106: f64, t116111: f64, t116115: f64, t116119: f64, t117447: f64, t117451: f64, t117461: f64, t119879: f64, t119883: f64, t119891: f64, t119901: f64, t122941: f64, t122945: f64, t122952: f64, t122955: f64, t122976: f64, t122979: f64, t124755: f64, t124778: f64, t124803: f64, t124805: f64, t124807: f64, t1409: f64, t31864: f64, t32331: f64, t32333: f64, t34126: f64, t3966: f64, t641: f64, t645: f64, t8308: f64, t84186: f64, t8513: f64, t8824: f64, t8825: f64) -> f64 {
    let t124814 = 5.0_f64 / 6.0_f64 * t116115 * t113875 * t124755 * t641 + 5.0_f64 / 18.0_f64 * t116111 * t34126 + 5.0_f64 / 18.0_f64 * t116119 * t34126 + 5.0_f64 / 18.0_f64 * t31864 * t8308 * t84186 * t1409 + 5.0_f64 / 18.0_f64 * t31864 * t8308 * t32331 * t3966 - 5.0_f64 / 9.0_f64 * t122941 * t8513 * t8824 * t1409 + 5.0_f64 / 18.0_f64 * t122945 * t32333 + 5.0_f64 / 6.0_f64 * t116115 * t113875 * t124778 * t645 + 5.0_f64 / 18.0_f64 * t122976 * t32333 - 35.0_f64 / 12.0_f64 * t122979 * t8308 * t124755 * t645 - 5.0_f64 / 3.0_f64 * t116106 * t117447 * t119879 - 5.0_f64 / 3.0_f64 * t116106 * t117447 * t119883 + 5.0_f64 / 9.0_f64 * t31864 * t117451 * t119891 + 5.0_f64 / 9.0_f64 * t31864 * t117451 * t119901 - 20.0_f64 / 27.0_f64 * t124803 + 5.0_f64 / 27.0_f64 * t124805 + 5.0_f64 / 27.0_f64 * t124807 - 5.0_f64 / 72.0_f64 * t122952 * t8825 - 5.0_f64 / 72.0_f64 * t122955 * t8825 - 10.0_f64 / 9.0_f64 * t117461;
    t124814
}
