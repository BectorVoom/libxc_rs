//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 435/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk435(t191: f64, t2331: f64, t369: f64, t371: f64, t364: f64, t367: f64, t899: f64, t912: f64) -> (f64, f64, f64) {
    let t2332 = t2331 * t191;
    let t2333 = t2332 * t369;
    let t2334 = t2333 * t371;
    let t2336 = 119.0_f64 / 13824.0_f64 * t364 * t2334;
    let t2343 = t899 * t912 * t367;
    (t2332, t2336, t2343)
}
