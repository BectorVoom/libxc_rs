//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1050/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1050(t124728: f64, t127720: f64, t127722: f64, t127726: f64, t127728: f64, t127730: f64, t127736: f64, t127738: f64, t127742: f64, t128289: f64, t128298: f64, t128300: f64, t128302: f64, t128303: f64, t129015: f64, t1459: f64, t2040: f64, t27863: f64, t33746: f64, t7796: f64, t7806: f64, t7941: f64) -> f64 {
    let t130302 = -4.0_f64 * t124728 * t1459 - 4.0_f64 * t129015 * t2040 - 4.0_f64 * t27863 * t7796 - 4.0_f64 * t27863 * t7806 + 2.0_f64 * t33746 * t7941 - t127720 - t127722 - t127726 - t127728 - t127730 + t127736 - t127738 - t127742 - t128289 - t128298 - t128300 - t128302 + t128303;
    t130302
}
