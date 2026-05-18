//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 924/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk924<F: Float>(t3652: F, t7741: F, t3657: F, t355: F, t879: F, t1095: F, t7457: F, t7458: F, t2104: F, t7780: F, t2067: F, t3073: F) -> (F, F, F, F, F, F) {
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    let t31539 = t355 * t879;
    let t31542 = t7457 * t7458 * t1095 * t31539;
    let t31544 = t7780 * t2104;
    let t31562 = t3073 * t2067;
    (t31530, t31532, t31539, t31542, t31544, t31562)
}
