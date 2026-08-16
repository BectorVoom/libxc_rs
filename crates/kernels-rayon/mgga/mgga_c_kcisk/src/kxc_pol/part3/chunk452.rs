//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 452/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk452(t3558: f64, t3559: f64, t457: f64, t306: f64, t416: f64, t1175: f64, t425: f64, t1364: f64, t298: f64, t301: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3560 = t3558 * t3559;
    let t3561 = t457 * t3560;
    let t3564 = t416 * t306;
    let t3565 = t425 * t1175;
    let t3566 = t3565 * t1364;
    let t3567 = t3564 * t3566;
    let t3571 = t298 * t446 * t301;
    (t3560, t3561, t3564, t3565, t3566, t3567, t3571)
}
