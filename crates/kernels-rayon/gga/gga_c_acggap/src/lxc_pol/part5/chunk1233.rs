//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1233/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1233(t13696: f64, t13699: f64, t13701: f64, t13706: f64, t13714: f64, t13729: f64, t13810: f64, t13812: f64, t16230: f64, t21707: f64, t21709: f64, t21712: f64, t21714: f64, t21717: f64) -> f64 {
    let t22575 = -40.0_f64 / 27.0_f64 * t13696 + 4.0_f64 / 3.0_f64 * t13699 + t13701 / 6.0_f64 + t13706 / 6.0_f64 - t13714 / 12.0_f64 + t13810 - t13729 / 3.0_f64 + t13812 + 2.0_f64 / 3.0_f64 * t21707 + 14.0_f64 / 9.0_f64 * t21709 + t21712 - 7.0_f64 / 9.0_f64 * t21714 - t21717 / 4.0_f64 + 2.0_f64 / 3.0_f64 * t16230;
    t22575
}
