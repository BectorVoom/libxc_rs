//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1166/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1166(t139507: f64, t139519: f64, t139526: f64, t140053: f64, t148640: f64, t148643: f64, t148646: f64, t148649: f64, t148653: f64, t148657: f64, t148660: f64, t148667: f64, t148670: f64, t148673: f64, t148676: f64, t148681: f64) -> f64 {
    let t148856 = t148640 - 2.0_f64 / 3.0_f64 * t148643 + 2.0_f64 / 3.0_f64 * t148646 - 2.0_f64 / 9.0_f64 * t148649 + 2.0_f64 / 3.0_f64 * t148653 - 6.0_f64 * t148657 + t148660 / 6.0_f64 - t139507 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t139519 + t139526 / 6.0_f64 - t140053 + t148667 + 4.0_f64 / 3.0_f64 * t148670 - 4.0_f64 / 9.0_f64 * t148673 - t148676 + t148681;
    t148856
}
