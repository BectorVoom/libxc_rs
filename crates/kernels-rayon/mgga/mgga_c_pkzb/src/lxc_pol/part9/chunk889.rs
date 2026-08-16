//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 889/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk889(t5728: f64, t6517: f64, t6461: f64, t758: f64, t2362: f64, t5717: f64) -> (f64, f64, f64, f64) {
    let t6518 = t5728 * t6517;
    let t6519 = t6461 * t6518;
    let t6520 = t758 * t6519;
    let t6523 = t5717 * t2362;
    (t6518, t6519, t6520, t6523)
}
