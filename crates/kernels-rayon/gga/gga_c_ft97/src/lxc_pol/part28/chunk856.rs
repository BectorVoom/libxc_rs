//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 856/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk856(t447: f64, t7288: f64, t925: f64, t32433: f64, t32463: f64, t32469: f64, t34624: f64, t34629: f64, t34634: f64, t34637: f64, t34640: f64, t34644: f64, t34649: f64, t34653: f64, t446: f64) -> (f64, f64) {
    let t34657 = t447 * t7288 * t925;
    let t34660 = t32433 + 2.0_f64 / 3.0_f64 * t446 * t34624 + t446 * t34629 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t34634 - 2.0_f64 * t446 * t34637 + 4.0_f64 / 3.0_f64 * t446 * t34640 + t32463 + 2.0_f64 / 3.0_f64 * t446 * t34644 + 2.0_f64 / 3.0_f64 * t446 * t34649 + 4.0_f64 / 3.0_f64 * t446 * t34653 + t32469 - t446 * t34657 / 9.0_f64;
    (t34657, t34660)
}
