//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2224/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224(t46137: f64, t40667: f64, t40670: f64, t40673: f64, t40680: f64, t40682: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40679: f64, t40685: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46138 = 0.32530743900905219526e-1_f64 * t46137;
    let t46140 = 0.15584273195113317383e3_f64 * t40667;
    let t46141 = 0.18311447306006545054e-3_f64 * t40670;
    let t46142 = 3.0_f64 * t40673;
    let t46143 = 0.73245789224026180215e-3_f64 * t40680;
    let t46144 = 0.10526802520742363173e2_f64 * t40682;
    let t46145 = -t39309 + t39312 + t39316 + t39320 - t46140 - t46141 + t46142 - t40679 + t46143 + t46144 - t40685;
    (t46138, t46140, t46141, t46142, t46143, t46144, t46145)
}
