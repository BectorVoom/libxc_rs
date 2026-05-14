//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 927/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk927<F: Float>(t4698: F, t40033: F, t58708: F, t58719: F, t74068: F, t74126: F, t74143: F, t74148: F, t74153: F, t74162: F, t85454: F, t85458: F, t45662: F, t58730: F, t85463: F, t85467: F, t85472: F, t85476: F, t85481: F, t85485: F, t85489: F, t85493: F, t85498: F, t85504: F) -> (F, F, F) {
    let t86829 = t4698 * t4698;
    let t86850 = 0.22226000364197530865e-1 * t74162 - t40033 - 0.22226000364197530866e-1 * t58708 - 0.1333560021851851852e0 * t74126 + 0.88904001456790123462e-1 * t74068 + 0.1333560021851851852e0 * t74143 + 0.69147556688614540471e-1 * t74148 - 0.17780800291358024693e0 * t74153 - 0.29634667152263374488e-1 * t58719 - 0.10001700163888888889e0 * t85454 - 0.13335600218518518519e0 * t85458;
    let t86863 = 0.66678001092592592595e-1 * t85463 + 0.8890400145679012346e-1 * t85467 - 0.40006800655555555556e0 * t85472 + 0.60010200983333333334e0 * t85476 + 0.44452000728395061732e-1 * t58730 - 0.62232801019753086422e0 * t85481 + 0.31116400509876543211e0 * t85485 + 0.80013601311111111114e0 * t85489 - 0.80013601311111111114e0 * t85493 + 0.2469555596021947874e-1 * t45662 + 0.17286889172153635117e0 * t85498 + 0.16669500273148148149e-1 * t85504;
    (t86829, t86850, t86863)
}
