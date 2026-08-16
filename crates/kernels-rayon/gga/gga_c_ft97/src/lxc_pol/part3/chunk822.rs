//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 822/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk822(t1013: f64, t51: f64, t6: f64, t398: f64, t4702: f64, t8907: f64, t12449: f64, t12452: f64, t16763: f64, t16769: f64, t16773: f64, t16777: f64, t16780: f64, t16786: f64, t2001: f64, t3392: f64, t3393: f64, t3404: f64, t399: f64, t4675: f64, t4712: f64, t538: f64, t554: f64) -> f64 {
    let t16792 = t1013 * t6 * t51;
    let t16793 = t16792 * t398;
    let t16798 = t8907 * t4702;
    let t16802 = 8.0_f64 * t2001 * t16763 + 4.0_f64 * t3392 * t3393 * t3404 - 2.0_f64 * t2001 * t16769 * t538 + 2.0_f64 * t3392 * t16773 * t554 - 4.0_f64 * t2001 * t16777 + 0.60409133884038297798e0_f64 * t16780 * t399 - 0.60409133884038297798e0_f64 * t4712 * t399 - 0.1208182677680765956e1_f64 * t16786 * t399 + 0.1208182677680765956e1_f64 * t4675 * t399 + 0.24163653553615319119e1_f64 * t12449 * t16793 - 0.24163653553615319119e1_f64 * t12452 * t16793 - 6.0_f64 * t3392 * t16798 * t554;
    t16802
}
