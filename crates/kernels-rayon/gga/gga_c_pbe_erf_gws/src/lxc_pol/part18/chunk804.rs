//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 804/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk804(t3205: f64, t336: f64, t2153: f64, t837: f64, t863: f64, t2262: f64, t344: f64, t362: f64, t2209: f64, t825: f64, t346: f64, t6158: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6523 = t3205 * t336;
    let t6542 = t863 * t2153 * t837;
    let t6552 = 1.0_f64 / t2262 / t344;
    let t6553 = t6552 * t362;
    let t6560 = t825 * t2209;
    let t6566 = t6158 * t346;
    (t6523, t6542, t6552, t6553, t6560, t6566)
}
