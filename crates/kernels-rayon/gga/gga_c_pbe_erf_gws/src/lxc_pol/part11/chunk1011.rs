//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1011/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1011(t2756: f64, t3399: f64, t10878: f64, t2741: f64, t12710: f64, t582: f64, t616: f64, t11019: f64, t7527: f64, t2749: f64, t3493: f64, t12517: f64, t184: f64, t202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40766 = t3399 * t2756;
    let t40768 = t2741 * t10878;
    let t40771 = t616 * t582 * t12710;
    let t40773 = t7527 * t11019;
    let t40783 = t3493 * t2749;
    let t40790 = t202 * t12517 * t184;
    (t40766, t40768, t40771, t40773, t40783, t40790)
}
