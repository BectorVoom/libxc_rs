//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1216/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1216<F: Float>(t7116: F, t8232: F, t29155: F, t46862: F, t29073: F, t8392: F, t24980: F, t24981: F, t28816: F, t684: F, t28821: F, t15533: F, t6334: F, t24976: F, t6317: F, t112807: F) -> (F, F, F, F, F, F, F, F) {
    let t113007 = t8232 * t7116;
    let t113009 = t46862 * t29155;
    let t113017 = 4.0 / 3.0 * t8392 * t29073;
    let t113035 = t24980 * t24981 * t28816 * t684;
    let t113039 = t24980 * t24981 * t28821 * t684;
    let t113041 = t6334 * t15533;
    let t113043 = t6317 * t24976 * t113041;
    let t113046 = t6317 * t24981 * t112807;
    (t113007, t113009, t113017, t113035, t113039, t113041, t113043, t113046)
}
