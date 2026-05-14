//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 527/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk527<F: Float>(t1963: F, t22: F, t161: F, t151: F, t177: F, t415: F, t968: F, t1077: F, t145: F, t334: F, t986: F, t339: F, t366: F, t374: F, t1137: F, t1145: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3558 = 1.0 / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = 0.37792653007779990369e-1 * t151 * t3559 * t177;
    let t3563 = t415 * t968;
    let t3565 = t1077 * t145;
    let t3570 = t986 * t334;
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3576 = t1137 * t1145;
    (t3558, t3562, t3563, t3565, t3570, t3571, t3573, t3574, t3576)
}
