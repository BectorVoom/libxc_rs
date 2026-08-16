//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 709/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk709(t2853: f64, t2908: f64, t141: f64, t2858: f64, t930: f64, t2862: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2892: f64, t2898: f64, t2900: f64, t2905: f64, t2906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2909 = t2908 * t2853;
    let t2910 = t141 * t2909;
    let t2912 = t930 * t2858;
    let t2913 = t141 * t2912;
    let t2915 = t930 * t2862;
    let t2916 = t141 * t2915;
    let t2918 = -0.9494625e0_f64 * t2882 + 0.1898925e1_f64 * t2890 + t2892 + 0.19931111111111111111e0_f64 * t2848 - 0.19931111111111111111e0_f64 * t2855 + 0.59793333333333333334e0_f64 * t2860 - 0.29896666666666666667e0_f64 * t2864 + 0.15358125e0_f64 * t2898 + 0.3071625e0_f64 * t2900 + t2905 + 0.10954222222222222222e0_f64 * t2906 - 0.27385555555555555556e-1_f64 * t2910 + 0.16431333333333333333e0_f64 * t2913 - 0.82156666666666666667e-1_f64 * t2916;
    (t2909, t2910, t2912, t2913, t2915, t2916, t2918)
}
