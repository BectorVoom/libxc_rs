//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 764/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk764<F: Float>(t1083: F, t1089: F, t8564: F, t598: F, t355: F, t506: F, t7458: F, t1980: F, t527: F, t7712: F, t1413: F, t2118: F) -> (F, F, F, F, F, F, F) {
    let t8566 = t1089 * t1083 * t8564;
    let t8567 = t598 * t8566;
    let t8569 = t355 * t506;
    let t8571 = t7458 * t1083 * t8569;
    let t8572 = t1980 * t8571;
    let t8574 = t7712 * t527;
    let t8576 = t2118 * t1413;
    (t8566, t8567, t8569, t8571, t8572, t8574, t8576)
}
