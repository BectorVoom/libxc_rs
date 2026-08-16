//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 643/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk643(t1062: f64, t2931: f64, t2834: f64, t2891: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t2870: f64, t2878: f64, t2886: f64, t2888: f64, t2893: f64, t2897: f64, t2900: f64, t2903: f64) -> (f64, f64, f64, f64) {
    let t2932 = t2931 * t1062;
    let t2937 = 0.68863333333333333333e0_f64 * t2834;
    let t2944 = 0.17365833333333333333e0_f64 * t2891;
    let t2949 = -0.17648625e1_f64 * t2870 + 0.3529725e1_f64 * t2878 + t2937 - 0.34431666666666666666e0_f64 * t2836 - 0.34431666666666666667e0_f64 * t2843 + 0.103295e1_f64 * t2848 + 0.516475e0_f64 * t2852 + 0.31558125e0_f64 * t2886 + 0.6311625e0_f64 * t2888 + t2944 - 0.13892666666666666667e0_f64 * t2893 - 0.34731666666666666667e-1_f64 * t2897 + 0.20839e0_f64 * t2900 + 0.104195e0_f64 * t2903;
    (t2932, t2937, t2944, t2949)
}
