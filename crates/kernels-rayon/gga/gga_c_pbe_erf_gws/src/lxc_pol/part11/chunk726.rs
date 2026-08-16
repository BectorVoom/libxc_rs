//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk726(t11190: f64, t672: f64, t3459: f64, t679: f64, t230: f64, t11159: f64, t164: f64, t331: f64, t3379: f64, t551: f64, t553: f64, t3380: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11191 = t11190 * t672;
    let t11229 = t3459 * t679;
    let t11231 = t3459 * t230;
    let t11250 = t11159 * t164;
    let t11262 = t331 * t3379;
    let t11264 = t11262 * t551 * t553;
    let t11268 = t3380 * t547;
    (t11191, t11229, t11231, t11250, t11262, t11264, t11268)
}
