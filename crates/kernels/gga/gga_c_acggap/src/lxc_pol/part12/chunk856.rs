//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 856/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk856<F: Float>(t3036: F, t597: F, t137: F, t3037: F, t1089: F, t1095: F, t2113: F, t7780: F, t1967: F, t7681: F, t3652: F, t7741: F, t3657: F, t355: F, t879: F, t7457: F, t7458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31520 = t3036 * t597;
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    let t31526 = t7780 * t2113;
    let t31528 = t1967 * t7681;
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    let t31539 = t355 * t879;
    let t31542 = t7457 * t7458 * t1095 * t31539;
    (t31520, t31521, t31524, t31526, t31528, t31530, t31532, t31539, t31542)
}
