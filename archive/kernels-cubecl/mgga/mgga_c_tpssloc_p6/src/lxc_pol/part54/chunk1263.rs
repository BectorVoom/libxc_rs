//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1263/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1263<F: Float>(t10109: F, t7841: F, t193: F, t7859: F, t671: F, t7786: F, t12020: F, t7936: F, t7982: F, t2169: F, t214: F, t6624: F) -> (F, F, F, F, F, F, F) {
    let t101551 = t10109 * t7841;
    let t101840 = t193 * t7859;
    let t102344 = t7786 * t671;
    let t102466 = t12020 * t7936;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    (t101551, t101840, t102344, t102466, t104977, t105108, t112660)
}
