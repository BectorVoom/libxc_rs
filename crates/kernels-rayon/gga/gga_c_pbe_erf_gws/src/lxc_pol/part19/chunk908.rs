//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 908/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk908(t10068: f64, t133: f64, t10071: f64, t10065: f64, t10094: f64, t10096: f64, t10102: f64, t10106: f64, t10123: f64, t10126: f64, t10129: f64, t8238: f64, t8249: f64, t8252: f64) -> f64 {
    let t10168 = t133 * t10068;
    let t10170 = t133 * t10071;
    let t10176 = t10094 - t10096 - 0.1724255e1_f64 * t10168 + 0.57475166666666666667e0_f64 * t10170 - 0.1724255e1_f64 * t133 * t10065 - 0.34485099999999999999e1_f64 * t8238 - t10102 + t8249 - 0.15326711111111111111e1_f64 * t8252 - t10106 - t10123 + t10126 + t10129;
    t10176
}
