//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 939/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk939(t4039: f64, t4032: f64, t4024: f64, t3854: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3873: f64, t4030: f64, t4035: f64, t4037: f64, t4042: f64) -> (f64, f64, f64, f64) {
    let t5639 = 0.5848223622634646207e0_f64 * t4039;
    let t5640 = 4.0_f64 * t4032;
    let t5641 = 4.0_f64 * t4024;
    let t5642 = t3854 + t3859 - t3862 - t3867 + t3871 + t3873 - t4035 - t4037 - t5639 + t4042 + t4030 - t5640 - t5641;
    (t5639, t5640, t5641, t5642)
}
