//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1023/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1023(t10106: f64, t16156: f64, t10043: f64, t5542: f64, t674: f64, t2004: f64, t2007: f64, t1987: f64, t26144: f64, t6394: f64, t645: f64, t26157: f64, t6397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46830 = t16156 * t10106;
    let t46832 = t10043 * t5542;
    let t46833 = t46832 * t674;
    let t46834 = t46833 * t2004;
    let t46836 = t46833 * t2007;
    let t46838 = t46833 * t1987;
    let t46841 = t26144 * t645 * t6394;
    let t46844 = t26157 * t645 * t6397;
    (t46830, t46832, t46833, t46834, t46836, t46838, t46841, t46844)
}
