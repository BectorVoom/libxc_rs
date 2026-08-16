//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3501/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3501(t1041: f64, t19799: f64, t3172: f64, t11262: f64, t6301: f64, t11999: f64, t19826: f64, t3150: f64, t6307: f64, t3059: f64, t5819: f64, t11710: f64, t19725: f64, t4892: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66017 = t1041 * t3172 * t19799;
    let t66022 = t1041 * t11262 * t6301;
    let t66024 = t11999 * t19826;
    let t66029 = t3150 * t11262 * t6307;
    let t66037 = t5819 * t3059;
    let t66043 = t4892 * t11710 * t19725;
    (t66017, t66022, t66024, t66029, t66037, t66043)
}
