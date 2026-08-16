//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 710/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk710(t1775: f64, t3918: f64, t3911: f64, t2: f64, t9952: f64, t3914: f64, t1148: f64, t8282: f64, t3932: f64, t11717: f64, t3922: f64, t3936: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13306 = 4.0_f64 / 9.0_f64 * t1775 * t3918;
    let t13308 = 4.0_f64 / 27.0_f64 * t1775 * t3911;
    let t13313 = t9952 * t2;
    let t13329 = 2.0_f64 / 9.0_f64 * t1775 * t3914;
    let t13335 = t8282 * t1148;
    let t13338 = 4.0_f64 / 3.0_f64 * t1775 * t3932;
    let t13339 = t11717 * t3922;
    let t13345 = 2.0_f64 / 3.0_f64 * t458 * t3936;
    (t13306, t13308, t13313, t13329, t13335, t13338, t13339, t13345)
}
