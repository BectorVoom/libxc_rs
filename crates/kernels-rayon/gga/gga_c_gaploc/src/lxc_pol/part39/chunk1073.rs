//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1073/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1073(t2325: f64, t38413: f64, t882: f64, t883: f64, t12013: f64, t2317: f64, t6525: f64, t13725: f64, t2312: f64, t12116: f64, t2321: f64, t11981: f64, t2268: f64, t2343: f64, t6509: f64) -> (f64, f64, f64, f64, f64) {
    let t46884 = t882 * t2325 * t883 * t38413;
    let t46887 = t6525 * t12013 * t2317;
    let t46889 = t2312 * t13725;
    let t46892 = t882 * t12116 * t2321;
    let t46896 = t2268 * t2343 * t11981 * t6509;
    (t46884, t46887, t46889, t46892, t46896)
}
