//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1002/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1002(t10201: f64, t225: f64, t3459: f64, t679: f64, t230: f64, t11009: f64, t11014: f64, t11016: f64, t11018: f64, t11021: f64, t11024: f64, t11027: f64, t11031: f64, t11034: f64, t231: f64, t7873: f64, t7876: f64, t7880: f64, t7890: f64, t7905: f64) -> f64 {
    let t11226 = t10201 * t225;
    let t11229 = t3459 * t679;
    let t11231 = t3459 * t230;
    let t11233 = -t7873 - t7876 + t7880 + t7890 + t11009 + t11014 + t11016 - t7905 - t11018 + 4.0_f64 / 3.0_f64 * t11226 * t231 + 4.0_f64 / 3.0_f64 * t11229 + t11021 - t11024 - t11027 + 4.0_f64 / 3.0_f64 * t11231 + t11031 + t11034;
    t11233
}
