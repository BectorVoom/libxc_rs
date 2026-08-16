//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 461/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk461(t1428: f64, t3587: f64, t457: f64, t1354: f64, t458: f64, t1364: f64) -> (f64, f64, f64, f64) {
    let t3588 = t1428 * t3587;
    let t3589 = t457 * t3588;
    let t3592 = t458 * t1354;
    let t3593 = t1364 * t1364;
    (t3588, t3589, t3592, t3593)
}
