//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 898/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk898(t35071: f64, t35115: f64, t35179: f64, t35220: f64, t160: f64, t35149: f64, t1969: f64, t32879: f64, t925: f64, t1349: f64, t1362: f64, t149: f64, t32714: f64, t34980: f64, t35007: f64, t35012: f64, t35016: f64, t35022: f64, t35028: f64, t35034: f64, t35038: f64, t35188: f64, t35197: f64, t35207: f64, t5772: f64, t6580: f64, t6584: f64, t6618: f64, t6622: f64, t7309: f64, t7315: f64, t7346: f64) -> (f64, f64, f64, f64) {
    let t35222 = t35071 + t35115 + t35179 + t35220;
    let t35229 = t35149 * t160;
    let t35234 = t1969 * t32879 * t925;
    let t35237 = t1349 * t34980 / 3.0_f64 + t6580 * t7346 / 3.0_f64 + t35007 * t1362 / 6.0_f64 - 2.0_f64 / 3.0_f64 * t1349 * t35012 - t1349 * t35016 / 3.0_f64 - t32714 * t6584 / 18.0_f64 + t5772 * t35022 / 9.0_f64 + t7309 * t6618 / 6.0_f64 - t1349 * t35028 / 3.0_f64 + t7309 * t6622 / 6.0_f64 - t149 * t35222 + 4.0_f64 * t35197 - 12.0_f64 * t35207 + 8.0_f64 * t35038 + 8.0_f64 * t35034 - 2.0_f64 * t35188 + 2.0_f64 * t35229 - t6580 * t7315 / 3.0_f64 - t5772 * t35234 / 9.0_f64;
    (t35222, t35229, t35234, t35237)
}
