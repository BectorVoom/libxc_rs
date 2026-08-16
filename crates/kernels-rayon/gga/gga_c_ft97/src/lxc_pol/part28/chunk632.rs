//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 632/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk632(t23076: f64, t23081: f64, t25999: f64, t26004: f64, t26009: f64, t26014: f64, t26019: f64, t26022: f64, t26025: f64, t26029: f64, t26033: f64, t26036: f64) -> f64 {
    let t26111 = t25999 + t26004 + t26009 / 4.0_f64 + t26014 / 4.0_f64 + t26019 / 4.0_f64 - t26022 / 3.0_f64 - t26025 / 12.0_f64 - t26029 - t26033 / 2.0_f64 + t26036 / 6.0_f64 - t23076 + t23081 / 3.0_f64;
    t26111
}
