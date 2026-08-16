//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 574/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk574(t4366: f64, t4367: f64, t1528: f64, t422: f64, t1416: f64, t4360: f64) -> (f64, f64, f64, f64) {
    let t4368 = t4366 * t4367;
    let t4370 = t1528 * t422;
    let t4371 = t4370 * t1416;
    let t4373 = -t4360;
    (t4368, t4370, t4371, t4373)
}
