//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 877/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk877(t16762: f64, t7115: f64, t7505: f64, t4892: f64, t5218: f64, t5220: f64, t5529: f64, t5544: f64, t562: f64, t7068: f64, t5275: f64, t579: f64) -> (f64, f64, f64, f64, f64) {
    let t16765 = 32.0_f64 / 15.0_f64 * t7115 * t7505 * t16762;
    let t16768 = 32.0_f64 / 15.0_f64 * t5218 * t5220 * t4892;
    let t16771 = 32.0_f64 / 15.0_f64 * t5218 * t5220 * t5529;
    let t16775 = 32.0_f64 / 9.0_f64 * t5218 * t7068 * t562 * t5544;
    let t16777 = 8.0_f64 / 15.0_f64 * t579 * t5275;
    (t16765, t16768, t16771, t16775, t16777)
}
