//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 701/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk701(t3034: f64, t334: f64, t371: f64, t202: f64, t6665: f64, t1877: f64, t1915: f64, t193: f64, t2522: f64, t6670: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64, f64) {
    let t6739 = 1.0_f64 / t3034 / t334;
    let t6793 = t371 * t334;
    let t6794 = 1.0_f64 / t6793;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0_f64 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    (t6739, t6794, t6834)
}
