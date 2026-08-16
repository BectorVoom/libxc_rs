//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 900/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk900(t10024: f64, t801: f64, t169: f64, t301: f64, t3373: f64, t784: f64, t3379: f64, t532: f64, t159: f64, t285: f64, t142: f64, t3637: f64) -> (f64, f64, f64, f64) {
    let t10025 = t10024 * t801;
    let t10026 = 0.41076328840066666668e0_f64 * t10025;
    let t10029 = t169 * t784 * t3373 * t301;
    let t10033 = t532 * t3379;
    let t10035 = t10033 * t159 * t285;
    let t10037 = t142 * t3637;
    (t10026, t10029, t10035, t10037)
}
