//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1999/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1999<F: Float>(t7049: F, t786: F, t867: F, t2467: F, t2772: F, t689: F, t7014: F, t25338: F, t887: F, t2439: F, t25334: F, t7036: F, t820: F, t844: F) -> (F, F, F, F, F, F) {
    let t92921 = t786 * t7049 * t867;
    let t92922 = t92921 * t2467;
    let t92925 = t689 * t7014 * t2772;
    let t92930 = t689 * t25338 * t887;
    let t92935 = t2439 * t25334 * t887;
    let t92951 = t820 * t7036 * t844;
    (t92921, t92922, t92925, t92930, t92935, t92951)
}
