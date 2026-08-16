//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2230/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2230(t40736: f64, t10126: f64, t12854: f64, t1877: f64, t2522: f64, t2745: f64, t40732: f64, t4119: f64, t4307: f64, t46235: f64, t46237: f64, t46238: f64, t46239: f64, t46240: f64, t46245: f64, t46252: f64) -> (f64, f64) {
    let t46256 = 4.0_f64 * t40736;
    let t46257 = 9.0_f64 * t10126 * t2522 * t4119 - 3.0_f64 * t12854 * t1877 * t2745 - 9.0_f64 * t2522 * t4307 * t46240 - 9.0_f64 * t2522 * t4307 * t46252 - t40732 - t46235 + t46237 + t46238 - t46239 + t46245 + t46256;
    (t46256, t46257)
}
