//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1027/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1027(t1939: f64, t1986: f64, t7720: f64, t39141: f64, t9222: f64, t1528: f64, t236: f64, t3351: f64, t551: f64, t7248: f64, t1587: f64, t618: f64) -> (f64, f64, f64, f64) {
    let t46891 = t1986 * t1939;
    let t46892 = t7720 * t46891;
    let t46894 = t9222 * t39141;
    let t46899 = t3351 * t7248 * t236 * t1528 * t551;
    let t46904 = t3351 * t7248 * t236 * t618 * t1587;
    (t46892, t46894, t46899, t46904)
}
