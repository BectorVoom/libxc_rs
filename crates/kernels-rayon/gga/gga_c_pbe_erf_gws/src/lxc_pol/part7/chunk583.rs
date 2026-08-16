//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 583/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk583(t4408: f64, t825: f64, t822: f64, t2169: f64, t838: f64, t329: f64) -> (f64, f64, f64, f64) {
    let t4409 = t4408 * t825;
    let t4410 = t822 * t4409;
    let t4413 = t838 * t2169;
    let t4414 = t329 * t4413;
    (t4409, t4410, t4413, t4414)
}
