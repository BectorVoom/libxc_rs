//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 738/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk738<F: Float>(t157: F, t309: F, t406: F, t315: F, t7911: F, t2137: F, t159: F, t3874: F) -> (F, F, F, F) {
    let t7965 = t309 * t406 * t157;
    let t7987 = t315 * t7911;
    let t7990 = t2137 * t7911;
    let t8004 = t3874 * t159;
    (t7965, t7987, t7990, t8004)
}
