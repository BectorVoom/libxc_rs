//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 565/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk565(t407: f64, t2834: f64, t1049: f64, t1053: f64, t1052: f64, t417: f64, t412: f64, t2891: f64, t420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2912 = t407 * t407;
    let t2913 = 1.0_f64 / t2912;
    let t2917 = 0.22831111111111111111e-1_f64 * t2834;
    let t2925 = t1049 * t1053;
    let t2928 = t1052 * t417;
    let t2929 = 1.0_f64 / t2928;
    let t2930 = t412 * t2929;
    let t2937 = 0.68863333333333333333e0_f64 * t2834;
    let t2944 = 0.17365833333333333333e0_f64 * t2891;
    let t2953 = t1052 * t1052;
    let t2954 = 1.0_f64 / t2953;
    let t2955 = t412 * t2954;
    let t2956 = t420 * t420;
    (t2912, t2913, t2917, t2925, t2929, t2930, t2937, t2944, t2953, t2954, t2955, t2956)
}
