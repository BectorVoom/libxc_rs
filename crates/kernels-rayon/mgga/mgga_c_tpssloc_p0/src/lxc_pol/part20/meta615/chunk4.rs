//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2220/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220(t16: f64, t39031: f64, t39033: f64, t39035: f64, t39037: f64, t39039: f64, t12566: f64, t604: f64, t2239: f64, t3951: f64, t12571: f64, t12582: f64, t12719: f64, t1437: f64, t2240: f64, t2241: f64, t39043: f64, t39049: f64, t39054: f64, t3953: f64, t3958: f64, t4021: f64, t45986: f64, t46022: f64, t46050: f64, t46080: f64, t605: f64, t645: f64, t86: f64, t9239: f64, t9243: f64, t9342: f64) -> f64 {
    let t46085 = 12.0_f64 * t16;
    let t46086 = 0.1248e2_f64 * t39031;
    let t46087 = 0.7092e3_f64 * t39033;
    let t46088 = 0.27744e4_f64 * t39035;
    let t46089 = 420.0_f64 * t39037;
    let t46090 = 0.911232e4_f64 * t39039;
    let t46099 = t12566 * t604;
    let t46104 = t3951 * t2239;
    let t46114 = 60.0_f64 * t12571 * t9243 - 4.0_f64 * t605 * (t45986 + t46022 + t46050 + t46080) + (-t46085 + t46086 - t46087 + t46088 + t46089 - t46090 + t39043) * t86 - 360.0_f64 * t9239 * t4021 * t2241 + 20.0_f64 * t2240 * t1437 * t9342 - 12.0_f64 * t46099 * t645 - 360.0_f64 * t39054 * t12582 + 60.0_f64 * t46104 * t2241 + 60.0_f64 * t39049 * t3958 - 4.0_f64 * t3953 * t9342 + 60.0_f64 * t2240 * t12719 * t645;
    t46114
}
