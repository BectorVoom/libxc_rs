//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1805/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1805(t23110: f64, t23185: f64, t25237: f64, t23168: f64, t25307: f64, t25287: f64, t81651: f64, t22893: f64, t23164: f64, t25320: f64, t7521: f64, t81632: f64) -> (f64, f64, f64, f64, f64) {
    let t87601 = t23185 * t23110 * t25237;
    let t87603 = t23168 * t25307;
    let t87612 = t81651 * t23110 * t25287;
    let t87618 = t23164 * t22893 * t25320;
    let t87635 = t81632 * t7521;
    (t87601, t87603, t87612, t87618, t87635)
}
