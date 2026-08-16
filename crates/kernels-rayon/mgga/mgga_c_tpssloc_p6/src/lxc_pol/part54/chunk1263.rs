//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1263/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1263(t10109: f64, t7841: f64, t193: f64, t7859: f64, t671: f64, t7786: f64, t12020: f64, t7936: f64, t7982: f64, t2169: f64, t214: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101551 = t10109 * t7841;
    let t101840 = t193 * t7859;
    let t102344 = t7786 * t671;
    let t102466 = t12020 * t7936;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    (t101551, t101840, t102344, t102466, t104977, t105108, t112660)
}
