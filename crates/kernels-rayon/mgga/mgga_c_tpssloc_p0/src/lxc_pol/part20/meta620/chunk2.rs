//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2235/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235(t46317: f64, t40808: f64, t2749: f64, t776: f64, t12915: f64, t2522: f64, t39549: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t46313: f64, t46314: f64, t46315: f64) -> (f64, f64, f64) {
    let t46318 = 12.0_f64 * t46317;
    let t46319 = 12.0_f64 * t40808;
    let t46320 = t776 * t2749;
    let t46324 = 18.0_f64 * t12915 * t2522 * t46320 + t39549 + t40797 + t40799 + t40801 - t40803 + t46313 - t46314 + t46315 + t46318 + t46319;
    (t46318, t46319, t46324)
}
