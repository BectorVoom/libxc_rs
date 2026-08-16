//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 601/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk601(t3112: f64, t3128: f64, t3144: f64, t3579: f64, t3580: f64, t3588: f64, t3592: f64, t4809: f64, t4812: f64, t4814: f64, t4817: f64, t5667: f64) -> f64 {
    let t5673 = -t4809 - 0.2445e0_f64 * t4812 - 0.2282e1_f64 * t4814 - t4817 + t3579 - t3580 + 0.2445e0_f64 * t3112 - 0.12225e0_f64 * t3128 - t3588 - 0.1141e1_f64 * t3144 + t3592;
    let t5674 = t5667 + t5673;
    t5674
}
