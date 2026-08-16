//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3656/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656(t422: f64, t69044: f64, t69058: f64, t69072: f64, t69086: f64, t5104: f64, t3433: f64, t3435: f64, t1150: f64, t3384: f64, t16835: f64, t5105: f64) -> (f64, f64, f64, f64) {
    let t69090 = 0.621814e-1_f64 * (t69044 + t69058 + t69072 + t69086) * t422;
    let t69091 = t5104 * t5104;
    let t69094 = 0.32163958997385070134e2_f64 * t3433 * t69091 * t3435;
    let t69097 = 4.0_f64 * t3384 * t69091 * t1150;
    let t69099 = 4.0_f64 * t16835 * t5105;
    (t69090, t69094, t69097, t69099)
}
