//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1138/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1138<F: Float>(t25410: F, t99403: F, t25374: F, t98848: F, t1711: F, t2411: F, t10309: F, t1470: F, t1513: F, t94975: F, t7706: F, t95293: F) -> (F, F, F, F, F, F, F) {
    let t99404 = t99403 * t25410;
    let t99463 = t98848 * t25374;
    let t99466 = t99403 * t25374;
    let t100987 = t2411 * t1711;
    let t101252 = t10309 * t1470;
    let t101451 = t94975 * t1513;
    let t101783 = t95293 * t7706;
    (t99404, t99463, t99466, t100987, t101252, t101451, t101783)
}
