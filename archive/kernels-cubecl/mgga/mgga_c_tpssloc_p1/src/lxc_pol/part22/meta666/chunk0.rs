//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2221/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2221<F: Float>(t17297: F, t2932: F, t2860: F, t5737: F, t2841: F, t5689: F, t17471: F, t923: F, t17488: F, t892: F, t17292: F, t699: F) -> (F, F, F, F, F, F) {
    let t59895 = t17297 * t2932;
    let t59920 = t5737 * t2860;
    let t59959 = t5689 * t2841;
    let t59962 = t17471 * t923;
    let t59979 = t17488 * t892;
    let t60163 = t699 * t17292;
    (t59895, t59920, t59959, t59962, t59979, t60163)
}
