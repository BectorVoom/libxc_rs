//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1180/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180(t1314: f64, t9580: f64, t3741: f64, t2566: f64, t3732: f64, t12204: f64, t12214: f64, t792: f64, t118: f64, t12156: f64, t794: f64, t2229: f64, t59: f64, t60: f64) -> (f64, f64, f64, f64) {
    let t40406 = t9580 * t1314;
    let t40407 = t40406 * t3741;
    let t40409 = t2566 * t3732;
    let t40410 = t40409 * t12204;
    let t40412 = t792 * t12214;
    let t40415 = t40412 * t118 * t794 * t12156;
    let t40419 = t59 / t60 / t2229;
    (t40407, t40410, t40415, t40419)
}
