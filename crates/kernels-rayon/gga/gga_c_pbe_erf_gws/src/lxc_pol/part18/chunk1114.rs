//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1114/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1114(t2201: f64, t326: f64, t378: f64, t4009: f64, t4414: f64, t13952: f64, t886: f64) -> (f64, f64, f64, f64) {
    let t13987 = t326 * t2201;
    let t13988 = t13987 * t378;
    let t13989 = 35.0_f64 / 432.0_f64 * t13988;
    let t13999 = t4414 * t4009;
    let t14001 = t13952 * t886;
    (t13987, t13989, t13999, t14001)
}
