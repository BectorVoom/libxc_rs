//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 629/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk629(t211: f64, t4908: f64, t1750: f64, t636: f64, t1729: f64, t586: f64) -> (f64, f64, f64) {
    let t4910 = 16.0_f64 / 405.0_f64 * t211 * t4908;
    let t4911 = t1750 * t636;
    let t4912 = 4.0_f64 / 15.0_f64 * t4911;
    let t4913 = t1729 * t586;
    (t4910, t4912, t4913)
}
