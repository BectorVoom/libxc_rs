//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1019/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1019(t28813: f64, t8607: f64, t27188: f64, t7468: f64, t33234: f64, t28045: f64, t7042: f64, t33358: f64, t91655: f64, t33363: f64, t7754: f64, t2018: f64, t26161: f64, t26558: f64, t6463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t128377 = 2.0_f64 * t8607 * t28813;
    let t128381 = 4.0_f64 * t27188 * t7468;
    let t128383 = 4.0_f64 * t33234 * t7468;
    let t128385 = 4.0_f64 * t7042 * t28045;
    let t128387 = 6.0_f64 * t91655 * t33358;
    let t128393 = 2.0_f64 * t33363 * t7754;
    let t128397 = 2.0_f64 * t26161 * t26558 * t2018 * t6463;
    (t128377, t128381, t128383, t128385, t128387, t128393, t128397)
}
