//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 925/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk925<F: Float>(t2067: F, t3088: F, t1165: F, t15758: F, t604: F, t1181: F, t599: F, t1089: F, t2079: F, t30052: F, t368: F, t31539: F, t7457: F, t7458: F) -> (F, F, F, F, F) {
    let t31567 = t3088 * t2067;
    let t31570 = t31567 * t1165 * t604 * t15758;
    let t31593 = t31567 * t1181 * t599 * t15758;
    let t31597 = t2079 * t1089 * t368 * t30052;
    let t31601 = t7457 * t7458 * t368 * t31539;
    (t31567, t31570, t31593, t31597, t31601)
}
