//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 626/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk626(t25846: f64, t469: f64, t1317: f64, t28: f64, t376: f64, t6508: f64, t23081: f64, t23124: f64, t25999: f64, t26004: f64, t26009: f64, t26014: f64, t26019: f64, t26022: f64, t26025: f64, t26029: f64) -> (f64, f64, f64) {
    let t26031 = t469 * t25846;
    let t26033 = t1317 * t28 * t26031;
    let t26036 = t1317 * t376 * t6508;
    let t26039 = t25999 / 3.0_f64 + t26004 / 3.0_f64 + t26009 / 12.0_f64 + t26014 / 12.0_f64 + t26019 / 12.0_f64 - t26022 / 9.0_f64 - t26025 / 36.0_f64 - t26029 / 3.0_f64 - t26033 / 6.0_f64 + t26036 / 18.0_f64 - t23124 + t23081 / 9.0_f64;
    (t26033, t26036, t26039)
}
