//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 703/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk703(t2371: f64, t2409: f64, t3959: f64, t1173: f64, t894: f64, t3958: f64, t867: f64) -> (f64, f64, f64, f64) {
    let t3960 = t2409 * t2371;
    let t3961 = t3959 * t3960;
    let t3963 = t1173 * t894;
    let t3965 = t3958 * t867;
    (t3960, t3961, t3963, t3965)
}
