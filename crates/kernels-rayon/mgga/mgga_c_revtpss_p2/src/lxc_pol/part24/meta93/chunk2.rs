//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 539/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539(t2846: f64, t307: f64, t944: f64, t302: f64, t2904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2930 = 0.22831111111111111111e-1_f64 * t2846;
    let t2941 = t944 * t307;
    let t2942 = 1.0_f64 / t2941;
    let t2943 = t302 * t2942;
    let t2950 = 0.68863333333333333333e0_f64 * t2846;
    let t2957 = 0.17365833333333333333e0_f64 * t2904;
    let t2966 = t944 * t944;
    (t2930, t2941, t2942, t2943, t2950, t2957, t2966)
}
