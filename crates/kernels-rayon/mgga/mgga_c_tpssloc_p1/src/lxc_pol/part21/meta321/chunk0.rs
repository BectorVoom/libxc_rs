//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1689/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1689(t28: f64, t528: f64, t1294: f64, t9722: f64, t172: f64, t3681: f64, t763: f64, t2528: f64, t3691: f64, t9919: f64, t2663: f64, t3814: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12072 = 1.0_f64 / t528 / t28;
    let t12087 = 0.10389515463408878255e3_f64 * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12091 = t3691 * t2528;
    let t12094 = 0.35089341735807877242e1_f64 * t1294 * t9919;
    let t12097 = t3814 * t2663;
    (t12072, t12087, t12088, t12089, t12091, t12094, t12097)
}
