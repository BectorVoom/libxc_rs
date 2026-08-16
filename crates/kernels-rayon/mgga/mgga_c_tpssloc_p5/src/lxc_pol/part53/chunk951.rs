//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 951/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk951(t31632: f64, t6883: f64, t22724: f64, t31623: f64, t22716: f64, t8631: f64, t31631: f64, t6897: f64, t794: f64, t113987: f64, t114012: f64, t114031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    let t115434 = t22716 * t8631;
    let t115439 = t6897 * t794 * t31631;
    let t115450 = 7.0_f64 / 144.0_f64 * t113987;
    let t115458 = 7.0_f64 / 576.0_f64 * t114012;
    let t115463 = 0.32298204875312312682e-2_f64 * t114031;
    (t115430, t115432, t115434, t115439, t115450, t115458, t115463)
}
