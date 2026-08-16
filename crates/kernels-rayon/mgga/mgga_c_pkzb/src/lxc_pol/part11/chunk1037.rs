//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1037/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1037(t11500: f64, t6557: f64, t2370: f64, t3880: f64, t2970: f64, t6570: f64, t11369: f64, t133: f64, t945: f64, t1227: f64, t394: f64, t6591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11501 = t11500 * t6557;
    let t11506 = t2370 * t3880;
    let t11507 = t2970 * t11506;
    let t11510 = t11500 * t6570;
    let t11519 = t11369 * t133;
    let t11520 = t11519 * t945;
    let t11524 = t2970 * t1227 * t394;
    let t11527 = t11500 * t6591;
    (t11501, t11506, t11507, t11510, t11519, t11520, t11524, t11527)
}
