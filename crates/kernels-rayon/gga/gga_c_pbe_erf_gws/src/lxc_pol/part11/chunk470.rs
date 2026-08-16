//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 470/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk470(t159: f64, t285: f64, t3013: f64, t1109: f64, t817: f64, t1161: f64, t2376: f64, t830: f64, t829: f64) -> (f64, f64, f64) {
    let t3015 = t3013 * t159 * t285;
    let t3030 = t1109 * t817;
    let t3045 = t2376 * t1161;
    let t3046 = t830 * t3045;
    let t3047 = t829 * t3046;
    (t3015, t3030, t3047)
}
