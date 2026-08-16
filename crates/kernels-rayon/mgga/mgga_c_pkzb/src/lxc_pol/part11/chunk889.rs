//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 889/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk889(t9267: f64, t9328: f64, t9586: f64, t9631: f64, t158: f64, t3675: f64, t6000: f64, t799: f64, t2964: f64, t2989: f64, t2118: f64, t3694: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9633 = t9267 + t9328 + t9586 + t9631;
    let t9634 = t9633 * t158;
    let t9647 = t6000 * t3675;
    let t9648 = t9647 * t799;
    let t9651 = t2964 * t2989;
    let t9656 = t2118 * t3694;
    (t9633, t9634, t9647, t9648, t9651, t9656)
}
