//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1060/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1060(t2195: f64, t2345: f64, t3240: f64, t2264: f64, t899: f64, t923: f64, t3249: f64, t3219: f64, t3235: f64, t6636: f64, t6684: f64, t8884: f64, t904: f64) -> (f64, f64, f64, f64, f64) {
    let t9626 = t2345 * t3240 * t2195;
    let t9630 = t899 * t2264 * t923;
    let t9632 = 7.0_f64 / 384.0_f64 * t9630 * t3249;
    let t9634 = t3235 * t3219 * t2195;
    let t9637 = t6684 * t6636;
    let t9638 = t904 * t8884;
    (t9626, t9632, t9634, t9637, t9638)
}
