//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 571/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk571(t4351: f64, t4352: f64, t1523: f64, t418: f64, t1407: f64, t34: f64, t39: f64) -> (f64, f64, f64, f64) {
    let t4353 = t4351 * t4352;
    let t4355 = t1523 * t418;
    let t4356 = t4355 * t1407;
    let t4358 = t34 * t39;
    (t4353, t4355, t4356, t4358)
}
