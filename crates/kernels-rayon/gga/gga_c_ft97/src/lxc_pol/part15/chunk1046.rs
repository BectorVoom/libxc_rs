//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1046/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1046(t11269: f64, t1526: f64, t1527: f64, t1528: f64, t15567: f64, t15568: f64, t15575: f64, t19972: f64, t20022: f64, t20031: f64, t20039: f64, t20044: f64, t20107: f64, t20130: f64, t3088: f64, t38327: f64, t38355: f64, t38357: f64, t75878: f64, t75881: f64) -> f64 {
    let t86536 = t15567 * t15575 * t20039 / 2.0_f64 + t1526 * t1527 * t20107 / 2.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3088 * t38357 * t20022 - t15567 * t15568 * t20031 / 3.0_f64 - t38355 - t1526 * t3088 * t20130 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t11269 * t38327 * t20022 + 2.0_f64 * t19972 - t75878 / 6.0_f64 - t75881 / 9.0_f64 - t1526 * t1527 * t1528 * t20044 / 12.0_f64;
    t86536
}
