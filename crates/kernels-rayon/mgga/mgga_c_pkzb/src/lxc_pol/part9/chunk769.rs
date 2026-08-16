//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 769/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk769(t135: f64, t144: f64, t2718: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5196: f64, t5217: f64, t5324: f64, t5326: f64, t5329: f64, t5333: f64, t5338: f64, t5340: f64, t5344: f64, t5466: f64, t560: f64, t568: f64, t639: f64) -> f64 {
    let t5470 = t135 * t144 * t5466 * t639 + 3.0_f64 * t135 * t5217 * t560 + 18.0_f64 * t2718 * t5196 * t568 + t5028 + t5040 + t5066 - t5069 - t5073 - t5324 + t5326 - t5329 + t5333 - t5338 - t5340 - t5344;
    t5470
}
