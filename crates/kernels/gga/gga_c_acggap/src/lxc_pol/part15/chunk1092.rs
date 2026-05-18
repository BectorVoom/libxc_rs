//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1092/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1092<F: Float>(t17912: F, t2288: F, t31443: F, t5616: F, t1844: F, t1992: F, t7585: F, t7842: F, t1165: F, t6218: F, t7351: F, t7575: F) -> (F, F, F) {
    let t38871 = t31443 * t17912 * t2288 * t5616;
    let t38875 = t7585 * t7842 * t1992 * t1844;
    let t38879 = t7575 * t1165 * t7351 * t6218;
    (t38871, t38875, t38879)
}
