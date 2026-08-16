//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 644/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk644(t1062: f64, t2949: f64, t1052: f64, t412: f64, t420: f64, t2931: f64, t2834: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2950 = t2949 * t1062;
    let t2953 = t1052 * t1052;
    let t2954 = 1.0_f64 / t2953;
    let t2955 = t412 * t2954;
    let t2956 = t420 * t420;
    let t2957 = 1.0_f64 / t2956;
    let t2958 = t2931 * t2957;
    let t2961 = 0.12361111111111111111e-1_f64 * t2834;
    let t2966 = t2961 - 0.61805555555555555556e-2_f64 * t2836 - 0.61805555555555555555e-2_f64 * t2843 + 0.18541666666666666667e-1_f64 * t2848 + 0.92708333333333333333e-2_f64 * t2852;
    let t2967 = t2966 * t434;
    (t2950, t2953, t2954, t2955, t2956, t2957, t2958, t2961, t2966, t2967)
}
