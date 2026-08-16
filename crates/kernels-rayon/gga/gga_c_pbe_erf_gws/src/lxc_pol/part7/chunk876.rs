//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 876/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk876(t16710: f64, t16751: f64, t173: f64, t184: f64, t199: f64, t1673: f64, t1680: f64, t5373: f64, t636: f64, t422: f64, t661: f64, t1416: f64) -> (f64, f64, f64, f64) {
    let t16756 = 2.0_f64 / 15.0_f64 * t173 * (t16710 + t16751) * t184 * t199;
    let t16757 = t1680 * t1673;
    let t16758 = 16.0_f64 / 45.0_f64 * t16757;
    let t16759 = t5373 * t636;
    let t16760 = 16.0_f64 / 45.0_f64 * t16759;
    let t16761 = t422 * t661;
    let t16762 = t16761 * t1416;
    (t16756, t16758, t16760, t16762)
}
