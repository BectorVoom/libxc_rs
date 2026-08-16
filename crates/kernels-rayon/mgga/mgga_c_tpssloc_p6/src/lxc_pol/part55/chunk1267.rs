//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1267/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1267(t2165: f64, t26135: f64, t652: f64, t7423: f64, t24969: f64, t7467: f64, t27921: f64, t6534: f64, t24972: f64, t26542: f64, t26545: f64, t105108: f64, t7769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t123244 = t652 * t2165 * t26135;
    let t123272 = t7423 * t26135;
    let t123274 = t24969 * t7467;
    let t123282 = t27921 * t6534;
    let t123285 = t24972 * t26542;
    let t123287 = t24972 * t26545;
    let t123290 = t105108 * t7769;
    (t123244, t123272, t123274, t123282, t123285, t123287, t123290)
}
