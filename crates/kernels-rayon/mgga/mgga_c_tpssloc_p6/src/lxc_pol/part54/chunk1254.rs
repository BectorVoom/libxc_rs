//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1254/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1254(t1902: f64, t4233: f64, t254: f64, t799: f64, t225: f64, t25161: f64, t214: f64, t4265: f64, t25222: f64, t25220: f64, t10143: f64, t1081: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87620 = t1902 * t4233;
    let t87755 = t799 * t254;
    let t87758 = t25161 * t225;
    let t87782 = t214 * t4265;
    let t87810 = t25222 * t225;
    let t87837 = t25220 * t225;
    let t89849 = t10143 * t1081;
    (t87620, t87755, t87758, t87782, t87810, t87837, t89849)
}
