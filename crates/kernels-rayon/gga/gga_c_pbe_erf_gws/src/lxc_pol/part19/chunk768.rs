//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 768/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk768(t5174: f64, t1617: f64, t732: f64, t1672: f64, t611: f64, t185: f64, t108: f64, t615: f64, t267: f64) -> (f64, f64, f64, f64) {
    let t5175 = 1.0_f64 / t5174;
    let t5205 = t732 * t1617;
    let t5207 = t1672 * t611;
    let t5208 = t185 * t5207;
    let t5210 = t615 * t108;
    let t5211 = t5210 * t267;
    (t5175, t5205, t5208, t5211)
}
