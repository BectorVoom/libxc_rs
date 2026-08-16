//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 773/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk773(t254: f64, t542: f64, t252: f64, t245: f64, t713: f64, t1697: f64, t212: f64, t22: f64, t1923: f64, t707: f64, t256: f64, t1914: f64, t1918: f64) -> (f64, f64, f64, f64, f64) {
    let t5385 = t254 * t542;
    let t5387 = 8.0_f64 / 81.0_f64 * t252 * t5385;
    let t5390 = t245 * t713;
    let t5399 = 1.0_f64 / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5416 = t707 * t1923;
    let t5417 = t5416 * t256;
    let t5418 = t1914 * t1918;
    (t5387, t5390, t5400, t5417, t5418)
}
