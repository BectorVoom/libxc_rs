//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 865/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk865(t6122: f64, t6233: f64, t2277: f64, t861: f64, t356: f64) -> (f64, f64, f64) {
    let t6283 = t6122 * t6233;
    let t6287 = 1.0_f64 / t2277 / t861;
    let t6288 = t356 * t6287;
    (t6283, t6287, t6288)
}
