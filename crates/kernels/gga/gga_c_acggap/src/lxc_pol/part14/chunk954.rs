//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 954/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk954<F: Float>(t1089: F, t2090: F, t27338: F, t598: F, t30364: F, t6184: F, t1988: F, t9681: F, t1841: F, t7685: F, t1426: F, t429: F, t9536: F, t137: F, t5506: F, t368: F) -> (F, F, F, F, F, F, F) {
    let t38909 = t598 * t1089 * t27338 * t2090;
    let t38912 = t30364 * t6184;
    let t38914 = t1988 * t9681;
    let t38916 = t7685 * t1841;
    let t38920 = t598 * t1426 * t429 * t9536;
    let t38922 = t137 * t5506;
    let t38925 = t598 * t1426 * t368 * t38922;
    (t38909, t38912, t38914, t38916, t38920, t38922, t38925)
}
