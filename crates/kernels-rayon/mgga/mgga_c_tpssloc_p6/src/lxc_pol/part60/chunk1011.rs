//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1011/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1011(t127796: f64, t127833: f64, t127858: f64, t127883: f64, t127926: f64, t127947: f64, t128042: f64, t128072: f64, t870: f64, t1530: f64, t33476: f64, t1914: f64, t5544: f64) -> (f64, f64, f64, f64) {
    let t128075 = t127796 + t127833 + t127858 + t127883 + t127926 + t127947 + t128042 + t128072;
    let t128076 = t128075 * t870;
    let t128080 = t33476 * t1530;
    let t128086 = t1914 * t5544;
    (t128075, t128076, t128080, t128086)
}
