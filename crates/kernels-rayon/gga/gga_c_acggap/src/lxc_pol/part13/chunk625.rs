//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 625/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk625(t3132: f64, t4353: f64, t345: f64, t3112: f64, t3118: f64, t3122: f64, t3128: f64, t3130: f64, t3144: f64, t3146: f64, t3580: f64, t3588: f64, t3592: f64) -> (f64, f64) {
    let t4833 = t3132 * t4353;
    let t4834 = t345 * t4833;
    let t4837 = -t3580 + 0.489e0_f64 * t3112 + 0.12225e0_f64 * t3118 - 0.61125e-1_f64 * t3122 - 0.2445e0_f64 * t3128 - 0.978e0_f64 * t3130 - t3588 - 0.2282e1_f64 * t3144 - 0.22005e1_f64 * t4834 + 0.489e0_f64 * t3146 + t3592;
    (t4834, t4837)
}
