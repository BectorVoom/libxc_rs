//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1130/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1130<F: Float>(t11662: F, t2200: F, t6857: F, t2221: F, t3729: F, t11661: F, t23609: F, t23612: F, t829: F, t11640: F, t3235: F, t22866: F, t23624: F, t35813: F, t6181: F, t2152: F, t2208: F, t3649: F, t3739: F) -> (F, F, F, F, F, F, F) {
    let t35901 = t11662 * t2200 * t6857;
    let t35903 = t2221 * t3729;
    let t35907 = t11661 * t23609 * t829 * t23612;
    let t35909 = t3235 * t11640;
    let t35912 = t11662 * t829 * t22866;
    let t35915 = t35813 * t6181 * t23624;
    let t35919 = t3649 * t2152 * t2208 * t3739;
    (t35901, t35903, t35907, t35909, t35912, t35915, t35919)
}
