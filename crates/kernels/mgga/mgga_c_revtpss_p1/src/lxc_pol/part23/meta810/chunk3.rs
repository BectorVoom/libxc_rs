//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2655/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655<F: Float>(t2439: F, t6132: F, t6135: F, t19013: F, t698: F, t19016: F, t6138: F, t18960: F, t18963: F, t18966: F, t19077: F, t914: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63533 = t2439 * t6132;
    let t63538 = t2439 * t6135;
    let t63541 = t698 * t19013;
    let t63543 = t698 * t19016;
    let t63545 = t2439 * t6138;
    let t63547 = t698 * t18960;
    let t63549 = t698 * t18963;
    let t63551 = t698 * t18966;
    let t63610 = t19077 * t914;
    (t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610)
}
