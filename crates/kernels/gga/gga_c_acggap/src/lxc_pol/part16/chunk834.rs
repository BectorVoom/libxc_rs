//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 834/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk834<F: Float>(t154: F, t2096: F, t31035: F, t3036: F, t597: F, t137: F, t3037: F, t1089: F, t1095: F, t2113: F, t7780: F, t3652: F, t7741: F, t3657: F, t355: F, t879: F) -> (F, F, F, F, F, F, F, F) {
    let t31508 = t31035 * t154 * t2096;
    let t31509 = 0.52805208333333333333e0 * t31508;
    let t31520 = t3036 * t597;
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    let t31525 = 0.94344276868812456204e-3 * t31524;
    let t31526 = t7780 * t2113;
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    let t31539 = t355 * t879;
    (t31509, t31520, t31521, t31525, t31526, t31530, t31532, t31539)
}
