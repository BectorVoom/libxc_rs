//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 711/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk711(t290: f64, t2875: f64, t2924: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t941: f64, t945: f64, t307: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2925 = t290 * t290;
    let t2926 = 1.0_f64 / t2925;
    let t2927 = t2875 * t2926;
    let t2929 = 0.16081979498692535067e2_f64 * t2924 * t2927;
    let t2930 = 0.22831111111111111111e-1_f64 * t2846;
    let t2935 = t2930 + 0.11415555555555555555e-1_f64 * t2848 - 0.11415555555555555555e-1_f64 * t2855 + 0.34246666666666666666e-1_f64 * t2860 - 0.17123333333333333333e-1_f64 * t2864;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941)
}
