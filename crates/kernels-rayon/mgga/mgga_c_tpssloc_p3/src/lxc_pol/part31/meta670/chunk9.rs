//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1998/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998(t101138: f64, t101150: f64, t101951: f64, t102102: f64, t113: f64, t1442: f64, t15868: f64, t19451: f64, t1983: f64, t22574: f64, t24175: f64, t26161: f64, t26163: f64, t26558: f64, t26559: f64, t26870: f64, t26902: f64, t26906: f64, t26974: f64, t28821: f64, t28834: f64, t28969: f64, t29197: f64, t29377: f64, t29378: f64, t5107: f64, t650: f64, t6876: f64, t6879: f64, t6999: f64, t7050: f64, t7218: f64, t7685: f64, t7787: f64, t7940: f64, t91655: f64, t92169: f64, t96797: f64, t97875: f64, t97894: f64) -> f64 {
    let t102105 = -t1983 * t29377 * t6999 + 4.0_f64 * t26161 * t101138 * t26163 + 6.0_f64 * t22574 * t26558 * t97894 - 6.0_f64 * t91655 * t26974 - 2.0_f64 * t1442 * t26870 - t650 * t29197 + 3.0_f64 * t1983 * t101150 * t6879 - 2.0_f64 * t19451 * t7050 + t6876 * t29378 - 6.0_f64 * t26161 * t92169 * t97875 + t28821 * t7218 - 2.0_f64 * t1983 * t7940 * t15868 - 2.0_f64 * t7685 * t26902 - 2.0_f64 * t7787 * t5107 + 4.0_f64 * t96797 * t26559 + 6.0_f64 * t7685 * t26906 + 3.0_f64 * t1983 * t24175 * t28834 + 3.0_f64 * t6876 * t28969 - t113 * (t101951 + t102102);
    t102105
}
