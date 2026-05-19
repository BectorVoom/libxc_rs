//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 945/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk945<F: Float>(t137: F, t3037: F, t1089: F, t1095: F, t31520: F, t2113: F, t7780: F, t1967: F, t7681: F, t3652: F, t7741: F, t3657: F) -> (F, F, F, F, F, F) {
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    let t31525 = F::cast_from(0.94344276868812456204e-3_f64) * t31524;
    let t31526 = t7780 * t2113;
    let t31528 = t1967 * t7681;
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    (t31521, t31525, t31526, t31528, t31530, t31532)
}
