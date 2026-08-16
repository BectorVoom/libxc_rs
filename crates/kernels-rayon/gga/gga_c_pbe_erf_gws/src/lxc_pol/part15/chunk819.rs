//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 819/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk819(t6616: f64, t854: f64, t2087: f64, t2142: f64, t899: f64, t912: f64, t923: f64, t2348: f64, t2251: f64, t916: f64, t2250: f64, t814: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6617 = t854 * t6616;
    let t6624 = t2087 * t2142;
    let t6627 = t899 * t912 * t923;
    let t6628 = t6627 * t2348;
    let t6636 = t2251 * t916;
    let t6637 = t2250 * t6636;
    let t6638 = t875 * t814;
    (t6617, t6624, t6627, t6628, t6636, t6637, t6638)
}
