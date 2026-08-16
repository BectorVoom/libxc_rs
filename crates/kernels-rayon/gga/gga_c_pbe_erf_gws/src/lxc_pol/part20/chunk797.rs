//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 797/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk797(t2209: f64, t337: f64, t2118: f64, t2365: f64, t274: f64, t4394: f64, t828: f64, t2137: f64, t2132: f64, t2271: f64, t814: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6148 = t2209 * t337;
    let t6154 = t2118 * t2365;
    let t6158 = t4394 * t274;
    let t6183 = t2365 * t828;
    let t6184 = t6183 * t2137;
    let t6187 = t2271 * t2132;
    let t6196 = t816 * t814;
    (t6148, t6154, t6158, t6183, t6184, t6187, t6196)
}
