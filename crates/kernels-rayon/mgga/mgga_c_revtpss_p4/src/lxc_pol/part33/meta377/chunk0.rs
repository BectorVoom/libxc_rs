//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1418/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1418(t15125: f64, t15168: f64, t15191: f64, t15197: f64, t15127: f64, t300: f64, t4682: f64, t3215: f64, t4858: f64, t3090: f64, t4954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15435 = 0.39862222222222222222e0_f64 * t15125;
    let t15447 = 0.21908444444444444444e0_f64 * t15168;
    let t15457 = 0.19931111111111111111e0_f64 * t15191;
    let t15459 = 0.10954222222222222222e0_f64 * t15197;
    let t15483 = 0.41203703703703703704e-2_f64 * t15127;
    let t15484 = 0.12361111111111111111e-1_f64 * t15125;
    let t15485 = 0.61805555555555555556e-2_f64 * t15191;
    let t15503 = 0.23744444444444444444e-1_f64 * t15125;
    let t15504 = 0.11872222222222222222e-1_f64 * t15191;
    let t15547 = t300 * t4682;
    let t15583 = 0.28582678745379824648e-3_f64 * t4858 * t3215;
    let t15618 = t4954 * t3090;
    (t15435, t15447, t15457, t15459, t15483, t15484, t15485, t15503, t15504, t15547, t15583, t15618)
}
