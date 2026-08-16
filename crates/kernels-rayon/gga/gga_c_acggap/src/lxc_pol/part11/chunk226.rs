//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 226/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk226(t43: f64, t50: f64, t292: f64, t817: f64, t818: f64, t824: f64, t53: f64, t238: f64, t296: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t828 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t817 * t818 + 2.0_f64 / 3.0_f64 * t292 * t824);
    let t829 = 1.0_f64 / t53;
    let t830 = t238 * t238;
    let t833 = -t824;
    let t837 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t829 * t830 + 2.0_f64 / 3.0_f64 * t296 * t833);
    let t839 = t828 / 2.0_f64 + t837 / 2.0_f64;
    (t829, t830, t833, t839)
}
