//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1696/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696(t10208: f64, t10213: f64, t10254: f64, t2339: f64, t2340: f64, t2366: f64, t46143: f64, t46144: f64, t46146: f64, t46148: f64, t46150: f64, t46152: f64, t46154: f64, t46157: f64, t46158: f64, t46166: f64, t46228: f64, t655: f64, t69: f64) -> f64 {
    let t46232 = t46143 + 616.0_f64 / 27.0_f64 * t46144 + 44.0_f64 / 3.0_f64 * t46146 - 22.0_f64 / 3.0_f64 * t46148 + 8.0_f64 * t46150 - 8.0_f64 * t46152 + 4.0_f64 / 3.0_f64 * t46154 + 3.0_f64 * t69 * t46157 * t46158 - 9.0_f64 / 2.0_f64 * t69 * t10208 * t2340 * t2366 + 3.0_f64 / 4.0_f64 * t69 * t2339 * t46166 + t69 * t10213 * t10254 - t69 * t655 * t46228 / 8.0_f64;
    t46232
}
