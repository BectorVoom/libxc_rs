//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1884/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1884(t25054: f64, t81651: f64, t82074: f64, t1880: f64, t23196: f64, t25224: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t6552: f64, t82124: f64) -> (f64, f64, f64, f64, f64) {
    let t87873 = t81651 * t82074 * t25054;
    let t87893 = t1880 * t25224 * t23196;
    let t87898 = t23030 * t25205;
    let t87901 = t23164 * t82133 * t7479;
    let t87904 = t6552 * t82124 * t7479;
    (t87873, t87893, t87898, t87901, t87904)
}
