//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1074/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1074(t145607: f64, t145611: f64, t145615: f64, t145619: f64, t145621: f64, t145626: f64, t145628: f64, t145632: f64, t145636: f64, t145640: f64, t145644: f64, t145648: f64, t145652: f64, t145656: f64, t145661: f64, t145663: f64) -> f64 {
    let t145893 = t145607 / 18.0_f64 - t145611 / 6.0_f64 - t145615 / 8.0_f64 - 2.0_f64 * t145619 + t145621 / 27.0_f64 + t145626 / 18.0_f64 - t145628 / 27.0_f64 - t145632 / 6.0_f64 - 4.0_f64 * t145636 + 8.0_f64 * t145640 - 4.0_f64 * t145644 - 2.0_f64 * t145648 + 2.0_f64 / 9.0_f64 * t145652 + t145656 / 9.0_f64 + t145661 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t145663;
    t145893
}
