//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 736/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk736(t3837: f64, t6501: f64, t3765: f64, t6402: f64, t3816: f64, t6627: f64, t2319: f64, t3810: f64, t3792: f64, t6183: f64, t3116: f64, t2164: f64, t3880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11846 = t6501 * t3837;
    let t11852 = t6402 * t3765;
    let t11857 = t6627 * t3816;
    let t11864 = t2319 * t3810;
    let t11868 = t6183 * t3792;
    let t11869 = t3116 * t11868;
    let t11912 = t2164 * t3880;
    (t11846, t11852, t11857, t11864, t11868, t11869, t11912)
}
