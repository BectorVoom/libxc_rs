//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 635/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk635(t2944: f64, t954: f64, t2846: f64, t2904: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64) -> (f64, f64, f64, f64) {
    let t2945 = t2944 * t954;
    let t2950 = 0.68863333333333333333e0_f64 * t2846;
    let t2957 = 0.17365833333333333333e0_f64 * t2904;
    let t2962 = -0.17648625e1_f64 * t2882 + 0.3529725e1_f64 * t2890 + t2950 + 0.34431666666666666666e0_f64 * t2848 - 0.34431666666666666667e0_f64 * t2855 + 0.103295e1_f64 * t2860 - 0.516475e0_f64 * t2864 + 0.31558125e0_f64 * t2898 + 0.6311625e0_f64 * t2900 + t2957 + 0.13892666666666666667e0_f64 * t2906 - 0.34731666666666666667e-1_f64 * t2910 + 0.20839e0_f64 * t2913 - 0.104195e0_f64 * t2916;
    (t2945, t2950, t2957, t2962)
}
