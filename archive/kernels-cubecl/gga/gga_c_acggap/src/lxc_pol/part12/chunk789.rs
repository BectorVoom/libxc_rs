//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 789/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk789<F: Float>(t598: F, t8561: F, t137: F, t1487: F, t1083: F, t1089: F, t355: F, t506: F, t7458: F, t1980: F, t527: F, t7712: F) -> (F, F, F, F, F, F, F, F) {
    let t8562 = t598 * t8561;
    let t8564 = t137 * t1487;
    let t8566 = t1089 * t1083 * t8564;
    let t8567 = t598 * t8566;
    let t8569 = t355 * t506;
    let t8571 = t7458 * t1083 * t8569;
    let t8572 = t1980 * t8571;
    let t8574 = t7712 * t527;
    (t8562, t8564, t8566, t8567, t8569, t8571, t8572, t8574)
}
