//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 411/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk411(t1472: f64, t168: f64, t286: f64, t38: f64, t8: f64) -> (f64, f64, f64) {
    let t1473 = t168 * t1472;
    let t1475 = 0.53218817823353818195e-1_f64 * t1473 * t286;
    let t1477 = 1.0_f64 / t8 / t38;
    (t1473, t1475, t1477)
}
