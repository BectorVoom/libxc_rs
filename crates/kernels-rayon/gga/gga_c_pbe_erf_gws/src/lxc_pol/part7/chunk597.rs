//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 597/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk597(t41: f64, t4562: f64, t1602: f64, t700: f64, t1383: f64, t536: f64, t1477: f64, t6: f64) -> (f64, f64, f64, f64) {
    let t4563 = t41 * t4562;
    let t4566 = t1602 * t700;
    let t4568 = t536 * t1383;
    let t4573 = t6 * t1477;
    (t4563, t4566, t4568, t4573)
}
