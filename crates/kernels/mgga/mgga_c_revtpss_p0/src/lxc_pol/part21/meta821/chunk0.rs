//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3038/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3038<F: Float>(t1086: F, t4930: F, t994: F, t342: F, t378: F, t43471: F, t3154: F, t43350: F, t16565: F, t989: F, t1071: F, t12046: F) -> (F, F, F, F, F) {
    let t55934 = t994 * t1086 * t4930;
    let t55938 = t342 * t43471 * t378;
    let t55939 = t43350 * t3154;
    let t55944 = t989 * t16565;
    let t55948 = t342 * t12046 * t1071;
    (t55934, t55938, t55939, t55944, t55948)
}
