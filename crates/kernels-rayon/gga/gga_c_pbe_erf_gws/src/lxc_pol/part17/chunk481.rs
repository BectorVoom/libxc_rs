//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 481/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk481(t143: f64, t1533: f64, t131: f64, t137: f64, t142: f64, t510: f64, t475: f64, t522: f64, t481: f64, t525: f64, t169: f64, t301: f64, t745: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2026 = t143 * t1533;
    let t2029 = t131 * t131;
    let t2030 = 1.0_f64 / t2029;
    let t2031 = t2030 * t137;
    let t2032 = t142 * t510;
    let t2033 = t2031 * t2032;
    let t2035 = t475 * t522;
    let t2036 = t142 * t481;
    let t2037 = t525 * t2036;
    let t2042 = t169 * t784 * t745 * t301;
    (t2026, t2029, t2030, t2031, t2032, t2033, t2035, t2036, t2037, t2042)
}
