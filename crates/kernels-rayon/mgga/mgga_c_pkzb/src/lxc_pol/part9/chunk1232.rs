//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1232/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1232(t7716: f64, t7725: f64, t5984: f64, t7713: f64, t2064: f64, t2899: f64, t2902: f64, t2029: f64, t7575: f64, t154: f64, t2048: f64, t276: f64, t7350: f64) -> (f64, f64, f64, f64, f64) {
    let t21494 = t7725 * t7716;
    let t21496 = t5984 * t7713;
    let t21499 = t2899 * t2064 * t2902;
    let t21500 = 0.28582678745379824648e-3_f64 * t21499;
    let t21518 = t7575 * t2029;
    let t21527 = t276 * t154 * t2048 * t7350;
    (t21494, t21496, t21500, t21518, t21527)
}
