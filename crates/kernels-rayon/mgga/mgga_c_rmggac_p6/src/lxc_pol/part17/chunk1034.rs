//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1034/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1034(t2310: f64, t9087: f64, t2412: f64, t8597: f64, t1982: f64, t7428: f64, t9775: f64, t9735: f64, t2186: f64, t9790: f64, t46764: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46992 = t9087 * t2310;
    let t46995 = t2412 * t8597;
    let t46999 = t9775 * t7428 * t1982;
    let t47004 = t9735 * t7428 * t1982;
    let t47006 = t2186 * t9790;
    let t47008 = t739 * t46764;
    (t46992, t46995, t46999, t47004, t47006, t47008)
}
