//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 484/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk484(t2985: f64, t315: f64, t2846: f64, t2904: f64, t963: f64, t323: f64, t300: f64, t960: f64, t988: f64, t993: f64, t378: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2986 = 1.0_f64 / t2985;
    let t2987 = t315 * t2986;
    let t2994 = 0.40256666666666666667e0_f64 * t2846;
    let t3001 = 0.137975e0_f64 * t2904;
    let t3010 = t963 * t963;
    let t3011 = 1.0_f64 / t3010;
    let t3012 = t315 * t3011;
    let t3013 = t323 * t323;
    let t3014 = 1.0_f64 / t3013;
    let t3022 = t300 * t960;
    let t3037 = 0.11111111111111111111e-1_f64 * t2846;
    let t3046 = t988 * t993;
    let t3047 = t3046 * t378;
    let t3052 = t989 * t378;
    (t2986, t2987, t2994, t3001, t3011, t3012, t3014, t3022, t3037, t3046, t3047, t3052)
}
