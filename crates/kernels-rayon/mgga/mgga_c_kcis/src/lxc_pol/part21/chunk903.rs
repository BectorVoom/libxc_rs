//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 903/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk903(t13710: f64, t13713: f64, t13715: f64, t13717: f64, t13720: f64, t13723: f64, t13726: f64, t13729: f64, t13732: f64, t13735: f64, t13738: f64, t13742: f64, t9681: f64, t9683: f64, t9691: f64, t9700: f64, t9736: f64) -> f64 {
    let t13744 = -t9736 - 8.0_f64 / 27.0_f64 * t9691 + 2.0_f64 / 27.0_f64 * t9683 - 2.0_f64 / 9.0_f64 * t9700 + t9681 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t13710 + t13713 - t13715 + 22.0_f64 / 9.0_f64 * t13717 - 10.0_f64 / 27.0_f64 * t13720 + 4.0_f64 / 3.0_f64 * t13723 - 8.0_f64 / 9.0_f64 * t13726 - 2.0_f64 / 9.0_f64 * t13729 - 2.0_f64 * t13732 + 8.0_f64 / 3.0_f64 * t13735 + 2.0_f64 / 3.0_f64 * t13738 - 2.0_f64 / 3.0_f64 * t13742;
    t13744
}
