//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1009/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1009(t9337: f64, t9383: f64, t9390: f64, t9409: f64, t541: f64, t1175: f64, t3656: f64, t1528: f64, t2944: f64, t1563: f64, t2817: f64, t1115: f64, t3792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9411 = t9337 + t9383 + t9390 + t9409;
    let t9412 = t9411 * t541;
    let t9413 = t3656 * t1175;
    let t9415 = t1528 * t2944;
    let t9416 = t2817 * t1563;
    let t9417 = t1115 * t3792;
    (t9411, t9412, t9413, t9415, t9416, t9417)
}
