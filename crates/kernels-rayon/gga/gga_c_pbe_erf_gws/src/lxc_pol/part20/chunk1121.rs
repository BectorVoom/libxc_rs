//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1121/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1121(t366: f64, t6238: f64, t899: f64, t4033: f64, t888: f64, t360: f64, t56: f64, t837: f64, t863: f64) -> (f64, f64, f64) {
    let t14035 = t899 * t6238 * t366;
    let t14042 = t4033 * t888;
    let t14046 = t863 * t360 * t837 * t56;
    (t14035, t14042, t14046)
}
