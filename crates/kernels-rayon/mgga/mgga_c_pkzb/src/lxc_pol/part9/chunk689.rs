//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 689/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk689(t1238: f64, t927: f64, t1167: f64, t919: f64, t921: f64, t2381: f64, t179: f64, t2405: f64, t404: f64, t326: f64, t397: f64, t297: f64, t401: f64, t46: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3217 = t1238 * t927;
    let t3223 = t1167 * t919;
    let t3224 = t3223 * t921;
    let t3225 = t2381 * t3224;
    let t3229 = t179 * t2405 * t1167;
    let t3230 = t404 * t3229;
    let t3232 = t397 * t326;
    let t3234 = t401 * t297 * t46;
    (t3217, t3224, t3225, t3230, t3232, t3234)
}
