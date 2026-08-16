//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1248/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1248(t15416: f64, t1610: f64, t4590: f64, t4632: f64, t11134: f64, t11534: f64, t15127: f64, t15189: f64, t15503: f64, t15504: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64) {
    let t19060 = 2.0_f64 * t15416 * t1610;
    let t19062 = 2.0_f64 * t4590 * t4632;
    let t19077 = -t11534 - 0.79148148148148148147e-2_f64 * t11134 - 0.15829629629629629629e-1_f64 * t15189 + 0.79148148148148148147e-2_f64 * t15127 - t15503 + t15504 + 0.39574074074074074073e-2_f64 * t18919 - 0.19787037037037037037e-1_f64 * t18906 + 0.71233333333333333332e-1_f64 * t18911 - 0.23744444444444444444e-1_f64 * t18915 - 0.11872222222222222222e-1_f64 * t18924 - 0.10685e0_f64 * t18928 + 0.71233333333333333332e-1_f64 * t18932 + 0.5936111111111111111e-2_f64 * t18934 - 0.11872222222222222222e-1_f64 * t18939 + 0.35616666666666666666e-1_f64 * t18944 - 0.17808333333333333333e-1_f64 * t18948;
    (t19060, t19062, t19077)
}
