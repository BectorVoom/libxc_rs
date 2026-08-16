//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 319/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk319(t944: f64, t945: f64, t338: f64, t828: f64, t448: f64, t80: f64) -> (f64, f64, f64, f64) {
    let t946 = t944 * t945;
    let t1185 = t828 * t338;
    let t1214 = t448 * t80;
    let t1215 = 1.0_f64 / t1214;
    (t946, t1185, t1214, t1215)
}
