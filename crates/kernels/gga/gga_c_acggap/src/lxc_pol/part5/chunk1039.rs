//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1039/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1039<F: Float>(t13064: F, t500: F, t171: F, t3300: F, t3775: F, t4360: F, t1036: F, t1089: F, t1298: F, t175: F, t864: F, t1423: F, t3770: F) -> (F, F, F, F, F) {
    let t17902 = t13064 * t500;
    let t17912 = t171 * t3300;
    let t17921 = t3775 * t4360;
    let t17926 = t1036 * t1089 * t175 * t1298 * t864;
    let t17928 = t3770 * t1423;
    (t17902, t17912, t17921, t17926, t17928)
}
