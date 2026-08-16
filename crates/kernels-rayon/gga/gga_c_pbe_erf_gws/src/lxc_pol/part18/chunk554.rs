//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 554/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk554(t120: f64, t2873: f64, t102: f64, t156: f64, t974: f64, t496: f64, t481: f64, t978: f64, t128: f64, t10: f64, t501: f64, t395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2874 = t120 * t2873;
    let t2876 = 0.2923025e1_f64 * t102 * t2874;
    let t2878 = t156 * t974;
    let t2879 = t496 * t2878;
    let t2881 = t978 * t481;
    let t2885 = t128 * t2873;
    let t2886 = t10 * t2885;
    let t2890 = t501 * t978;
    let t2891 = t2890 * t395;
    (t2874, t2876, t2878, t2879, t2881, t2885, t2886, t2890, t2891)
}
