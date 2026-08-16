//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 894/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk894(t24215: f64, t3553: f64, t13350: f64, t4342: f64, t41609: f64, t41612: f64, t41618: f64, t41623: f64, t41626: f64, t41629: f64, t11371: f64, t2482: f64, t9267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46023 = 2.0_f64 * t24215 * t3553;
    let t46025 = 2.0_f64 * t4342 * t13350;
    let t46030 = 0.30674340763136599742e1_f64 * t41609;
    let t46031 = 0.14570311862489884877e2_f64 * t41612;
    let t46033 = 0.23833659967900284446e0_f64 * t41618;
    let t46035 = 0.11916829983950142223e0_f64 * t41623;
    let t46036 = 0.11916829983950142223e0_f64 * t41626;
    let t46037 = 0.11916829983950142223e0_f64 * t41629;
    let t46044 = t9267 * t11371 * t2482;
    (t46023, t46025, t46030, t46031, t46033, t46035, t46036, t46037, t46044)
}
