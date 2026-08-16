//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 993/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk993(t19451: f64, t8533: f64, t28002: f64, t33231: f64, t4028: f64, t28864: f64, t7042: f64, t33222: f64, t96797: f64, t28952: f64, t8526: f64, t29219: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127722 = 2.0_f64 * t19451 * t8533;
    let t127726 = 4.0_f64 * t28002 * t8533;
    let t127728 = 4.0_f64 * t4028 * t33231;
    let t127730 = 2.0_f64 * t7042 * t28864;
    let t127736 = 4.0_f64 * t96797 * t33222;
    let t127738 = 2.0_f64 * t8526 * t28952;
    let t127742 = 4.0_f64 * t8526 * t29219;
    (t127722, t127726, t127728, t127730, t127736, t127738, t127742)
}
