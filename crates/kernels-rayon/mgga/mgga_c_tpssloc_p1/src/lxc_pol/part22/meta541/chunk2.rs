//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2029/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2029(t39378: f64, t746: f64, t9720: f64, t1294: f64, t1285: f64, t9214: f64, t12132: f64, t588: f64, t39253: f64, t702: f64, t9453: f64) -> (f64, f64, f64, f64, f64) {
    let t39568 = t9720 * t39378 * t746;
    let t39570 = 0.14035736694323150897e2_f64 * t1294 * t39568;
    let t39571 = t9214 * t1285;
    let t39581 = t588 * t12132;
    let t39585 = 24.0_f64 * t9453 * t39253 * t702;
    (t39568, t39570, t39571, t39581, t39585)
}
