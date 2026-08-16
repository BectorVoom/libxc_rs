//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3213/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3213(t13334: f64, t13340: f64, t13393: f64, t13396: f64, t13397: f64, t13400: f64, t13405: f64, t1470: f64, t1486: f64, t1494: f64, t21707: f64, t21710: f64, t21713: f64, t2312: f64, t38: f64, t4181: f64, t4182: f64, t4187: f64, t4217: f64, t4238: f64, t5830: f64, t60937: f64, t60987: f64, t641: f64, t85: f64) -> f64 {
    let t60994 = -t13396 * t1486 * t85 / 3.0_f64 - t4181 * t4217 * t85 / 3.0_f64 - t21707 * t641 / 3.0_f64 - t13405 * t1486 * t85 / 6.0_f64 - t4187 * t4217 * t85 / 3.0_f64 - t21710 * t641 / 3.0_f64 - t1470 * t13334 * t85 / 6.0_f64 - t21713 * t641 / 3.0_f64 - t5830 * t2312 / 6.0_f64 - t13393 * t1494 / 6.0_f64 - t13397 * t1494 / 3.0_f64 - t13400 * t1494 / 3.0_f64 - t4182 * t4238 / 3.0_f64 + t38 * (t60937 + t60987) * t85 / 24.0_f64 - t13340 * t1494 / 6.0_f64;
    t60994
}
