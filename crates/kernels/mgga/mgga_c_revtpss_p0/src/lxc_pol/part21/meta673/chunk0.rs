//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2474/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2474<F: Float>(t1071: F, t11200: F, t378: F, t42358: F, t11223: F, t12032: F, t994: F, t3259: F, t989: F, t11213: F, t42277: F, t3376: F, t3383: F) -> (F, F, F, F, F, F, F, F) {
    let t43637 = t11200 * t1071;
    let t43642 = t42358 * t378;
    let t43656 = t11223 * t1071;
    let t43670 = t994 * t12032;
    let t43687 = t989 * t3259;
    let t43696 = t11213 * t1071;
    let t43707 = t42277 * t378;
    let t43748 = t3376 * t3383;
    (t43637, t43642, t43656, t43670, t43687, t43696, t43707, t43748)
}
