//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2241/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2241<F: Float>(t11631: F, t12050: F, t3151: F, t15907: F, t12077: F, t378: F, t342: F, t3154: F, t12046: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16553 = t12050 * t11631;
    let t16554 = t16553 * t3151;
    let t16555 = t15907 * t16554;
    let t16558 = t12077 * t378;
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    let t16561 = t16560 * t3151;
    let t16562 = t15907 * t16561;
    let t16565 = t12046 * t378;
    let t16566 = t342 * t16565;
    (t16553, t16554, t16555, t16558, t16559, t16560, t16561, t16562, t16565, t16566)
}
