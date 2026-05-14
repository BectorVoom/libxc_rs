//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 220/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk220<F: Float>(t729: F, t747: F, t31: F, t4: F, t668: F, t132: F, t200: F, t220: F, t721: F, t199: F, t27: F, t13: F, t218: F) -> (F, F, F, F, F, F, F, F) {
    let t748 = t729 * t747;
    let t752 = t4 * t668 * t31;
    let t753 = 0.14764627977777777777e-2 * t752;
    let t754 = t132 * t200;
    let t756 = t721 * t754 * t220;
    let t757 = 0.35616666666666666666e-1 * t756;
    let t758 = t199 * t27;
    let t759 = 1.0 / t758;
    let t760 = t13 * t759;
    let t761 = t218 * t218;
    (t748, t753, t754, t757, t758, t759, t760, t761)
}
