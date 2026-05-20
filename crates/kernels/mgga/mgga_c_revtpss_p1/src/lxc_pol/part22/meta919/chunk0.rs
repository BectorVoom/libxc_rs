//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3129/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3129<F: Float>(t12077: F, t1647: F, t1086: F, t4930: F, t994: F, t342: F, t378: F, t43471: F, t3154: F, t43350: F, t16565: F, t989: F) -> (F, F, F, F, F) {
    let t55899 = t1647 * t12077;
    let t55934 = t994 * t1086 * t4930;
    let t55938 = t342 * t43471 * t378;
    let t55939 = t43350 * t3154;
    let t55944 = t989 * t16565;
    (t55899, t55934, t55938, t55939, t55944)
}
