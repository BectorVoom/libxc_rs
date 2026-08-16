//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1294/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1294(t15494: f64, t324: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11534: f64, t15127: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64) {
    let t15495 = t15494 * t324;
    let t15503 = 0.23744444444444444444e-1_f64 * t15125;
    let t15504 = 0.11872222222222222222e-1_f64 * t15191;
    let t15513 = -t11534 - 0.15829629629629629629e-1_f64 * t11134 + 0.39574074074074074073e-2_f64 * t11136 - 0.11872222222222222222e-1_f64 * t11138 + 0.5936111111111111111e-2_f64 * t11140 - 0.79148148148148148146e-2_f64 * t15189 + 0.79148148148148148146e-2_f64 * t15127 - t15503 + t15504 - 0.19787037037037037037e-1_f64 * t15142 + 0.71233333333333333332e-1_f64 * t15156 - 0.23744444444444444444e-1_f64 * t15132 - 0.11872222222222222222e-1_f64 * t15137 - 0.10685e0_f64 * t15160 + 0.71233333333333333332e-1_f64 * t15147 + 0.35616666666666666666e-1_f64 * t15151 - 0.17808333333333333333e-1_f64 * t15195;
    (t15495, t15513)
}
