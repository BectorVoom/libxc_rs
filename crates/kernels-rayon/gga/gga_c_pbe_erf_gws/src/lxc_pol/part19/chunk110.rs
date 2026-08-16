//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 110/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk110(t201: f64, t223: f64, t252: f64, t256: f64, t265: f64, t267: f64, t33: f64, t89: f64, t91: f64) -> (f64, f64) {
    let t270 = t201 + t223 + t252 * t256 / 3.0_f64 - t265 * t267 / 15.0_f64;
    let t274 = -t33 + t89 + t91;
    (t270, t274)
}
