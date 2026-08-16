//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 626/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk626(t673: f64, t8664: f64, t716: f64, t720: f64, t415: f64, t2527: f64) -> (f64, f64, f64, f64) {
    let t8665 = t673 * t8664;
    let t8666 = t8665 * t716;
    let t8667 = t8666 * t720;
    let t8668 = t415 * t8667;
    let t8672 = t2527 * t2527;
    (t8666, t8667, t8668, t8672)
}
