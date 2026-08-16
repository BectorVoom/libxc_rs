//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 937/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk937(t10189: f64, t405: f64, t921: f64, t758: f64, t3857: f64, t754: f64, t46: f64, t915: f64) -> (f64, f64, f64, f64) {
    let t10191 = t405 * t10189 * t921;
    let t10192 = t758 * t10191;
    let t10195 = t3857 * t754;
    let t10196 = t10195 * t46;
    let t10197 = t915 * t10196;
    (t10191, t10192, t10195, t10197)
}
