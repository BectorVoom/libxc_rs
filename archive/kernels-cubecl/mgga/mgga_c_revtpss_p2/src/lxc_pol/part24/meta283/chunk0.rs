//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1059/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1059<F: Float>(t11465: F, t6189: F, t3336: F, t6396: F, t6184: F, t964: F, t6152: F, t945: F, t11387: F, t6109: F, t2970: F, t6173: F) -> (F, F, F, F, F, F) {
    let t19133 = t11465 * t6189;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    let t19173 = t6152 * t945;
    let t19255 = t6109 * t11387;
    let t19275 = t6173 * t2970;
    (t19133, t19153, t19156, t19173, t19255, t19275)
}
