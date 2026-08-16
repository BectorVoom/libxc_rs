//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1018/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1018(t1756: f64, t7778: f64, t305: f64, t45418: f64, t5271: f64, t46258: f64, t5162: f64, t46415: f64, t4669: f64, t1704: f64, t2124: f64, t27048: f64, t46541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46764 = t7778 * t1756;
    let t46765 = t305 * t46764;
    let t46770 = t5271 * t45418;
    let t46772 = t5162 * t46258;
    let t46774 = t4669 * t46415;
    let t46779 = t2124 * t1704;
    let t46782 = t27048 * t46541;
    (t46764, t46765, t46770, t46772, t46774, t46779, t46782)
}
