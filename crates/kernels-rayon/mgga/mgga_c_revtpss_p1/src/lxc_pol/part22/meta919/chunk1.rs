//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3130/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3130(t1071: f64, t12046: f64, t342: f64, t3298: f64, t4743: f64, t3316: f64, t19602: f64, t994: f64, t19607: f64, t12166: f64, t1647: f64, t4746: f64, t4980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55948 = t342 * t12046 * t1071;
    let t55958 = t4743 * t3298;
    let t55985 = t4743 * t3316;
    let t55988 = t994 * t19602;
    let t55991 = t994 * t19607;
    let t56017 = t1647 * t12166;
    let t56049 = t4746 * t4980;
    (t55948, t55958, t55985, t55988, t55991, t56017, t56049)
}
