//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 176/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk176(t546: f64, t83: f64, t124: f64, t512: f64, t46: f64, t99: f64, t123: f64, t465: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t547 = t83 * t546;
    let t549 = 0.19751673498613801407e-1_f64 * t512 * t124;
    let t550 = t99 * t46;
    let t552 = t475 * t465 * t123;
    (t547, t549, t550, t552)
}
