//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 995/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk995(t3749: f64, t6717: f64, t20378: f64, t3912: f64, t11777: f64, t6183: f64, t20940: f64, t3837: f64, t1114: f64, t3747: f64, t6670: f64, t3871: f64, t6505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36920 = t6717 * t3749;
    let t36962 = t3912 * t20378;
    let t37138 = t6183 * t11777;
    let t37257 = t20940 * t3837;
    let t37286 = t1114 * t3747 * t6670;
    let t37363 = t6505 * t3871;
    (t36920, t36962, t37138, t37257, t37286, t37363)
}
