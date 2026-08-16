//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 530/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk530(t2918: f64, t935: f64, t915: f64, t913: f64, t275: f64, t290: f64, t2875: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2919 = t2918 * t935;
    let t2921 = 1.0_f64 * t915 * t2919;
    let t2922 = t913 * t913;
    let t2923 = 1.0_f64 / t2922;
    let t2924 = t275 * t2923;
    let t2925 = t290 * t290;
    let t2926 = 1.0_f64 / t2925;
    let t2927 = t2875 * t2926;
    let t2929 = 0.16081979498692535067e2_f64 * t2924 * t2927;
    let t2930 = 0.22831111111111111111e-1_f64 * t2846;
    let t2935 = t2930 + 0.11415555555555555555e-1_f64 * t2848 - 0.11415555555555555555e-1_f64 * t2855 + 0.34246666666666666666e-1_f64 * t2860 - 0.17123333333333333333e-1_f64 * t2864;
    (t2919, t2921, t2922, t2923, t2924, t2925, t2926, t2927, t2929, t2935)
}
