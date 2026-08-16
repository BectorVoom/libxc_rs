//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1341/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1341(t39054: f64, t7245: f64, t50: f64, t9300: f64, t10913: f64, t1860: f64, t1864: f64, t2109: f64, t2110: f64, t22489: f64, t22493: f64, t22546: f64, t24498: f64, t24504: f64, t24505: f64, t24511: f64, t6486: f64, t6495: f64, t6509: f64, t67: f64, t7246: f64, t7251: f64, t7255: f64, t7256: f64, t7259: f64, t83699: f64, t83706: f64, t83710: f64, t83771: f64, t83803: f64, t9258: f64, t9288: f64) -> f64 {
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85569 = 5.0_f64 / 2.0_f64 * t7246 * t83771 + t6495 * t24511 - 15.0_f64 * t85536 * t22546 - t1860 * (5.0_f64 / 108.0_f64 * t85539 * t9288 + 5.0_f64 / 6.0_f64 * t24498 * t10913 - 5.0_f64 / 6.0_f64 * t7251 * t9258 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t24504 * t6509 / 2.0_f64 - t1860 * t7255 * t22489 / 2.0_f64 - t1860 * t2109 * t83706 / 6.0_f64 + t83699 * t2110 - t83710 * t2110 / 6.0_f64 - t22493 * t7256 / 2.0_f64 - t22493 * t7259 / 2.0_f64 - t6486 * t24505 / 2.0_f64;
    t85569
}
