//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1139/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1139(t23097: f64, t2628: f64, t2632: f64, t47320: f64, t46519: f64, t6605: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64, t46511: f64, t815: f64) -> (f64, f64, f64, f64) {
    let t81728 = t23097 * t2628 * t47320 * t2632;
    let t81731 = t6605 * t2628 * t46519;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81738 = t6605 * t815 * t46511;
    (t81728, t81731, t81735, t81738)
}
