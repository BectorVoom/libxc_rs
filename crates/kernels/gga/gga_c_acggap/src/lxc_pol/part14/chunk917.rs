//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 917/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk917<F: Float>(t31524: F, t2113: F, t7780: F, t3652: F, t7741: F, t3657: F, t355: F, t879: F, t1095: F, t7457: F, t7458: F, t2104: F) -> (F, F, F, F, F, F, F) {
    let t31525 = F::new(0.94344276868812456204e-3) * t31524;
    let t31526 = t7780 * t2113;
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    let t31539 = t355 * t879;
    let t31542 = t7457 * t7458 * t1095 * t31539;
    let t31543 = F::new(0.31448092289604152067e-3) * t31542;
    let t31544 = t7780 * t2104;
    (t31525, t31526, t31530, t31532, t31539, t31543, t31544)
}
