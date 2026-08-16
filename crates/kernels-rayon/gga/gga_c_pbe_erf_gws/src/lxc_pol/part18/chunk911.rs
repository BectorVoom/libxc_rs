//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 911/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk911(t101: f64, t10179: f64, t10029: f64, t10035: f64, t10037: f64, t10046: f64, t2857: f64, t2922: f64, t2986: f64, t3642: f64, t481: f64, t526: f64, t8267: f64, t8270: f64, t8275: f64, t8277: f64, t8281: f64, t8290: f64, t8293: f64, t8302: f64, t8318: f64, t8497: f64) -> f64 {
    let t10180 = t101 * t10179;
    let t10185 = -0.54045904796391420712e-1_f64 * t10029 + 6.0_f64 * t2986 * t8302 - 0.29056741517886919367e-3_f64 * t10035 + 6.0_f64 * t2857 * t10037 * t481 + t2922 * t3642 - 6.0_f64 * t8497 * t8293 + 6.0_f64 * t2986 * t8318 + 12.0_f64 * t2857 * t10046 + t10180 * t526 - t8267 - 0.23948468020509218188e-1_f64 * t8270 - t8275 - 0.11622696607154767747e-2_f64 * t8277 + 0.27119625416694458076e-2_f64 * t8281 - t8290;
    t10185
}
