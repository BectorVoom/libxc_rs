//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 686/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk686(t5513: f64, t1678: f64, t577: f64, t184: f64, t199: f64, t266: f64, t331: f64, t265: f64, t1640: f64, t649: f64, t1692: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5514 = 4.0_f64 / 45.0_f64 * t5513;
    let t5515 = t1678 * t577;
    let t5516 = t5515 * t184;
    let t5518 = 4.0_f64 / 5.0_f64 * t5516 * t199;
    let t5519 = t266 * t331;
    let t5521 = 8.0_f64 / 405.0_f64 * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5523 = t1692 * t661;
    (t5514, t5515, t5516, t5518, t5519, t5521, t5522, t5523)
}
