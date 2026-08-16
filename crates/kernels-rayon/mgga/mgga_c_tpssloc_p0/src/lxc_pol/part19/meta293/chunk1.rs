//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1070/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1070(t1388: f64, t3698: f64, t3700: f64, t570: f64, t11976: f64, t11978: f64, t11980: f64, t11982: f64, t11984: f64, t12012: f64, t12044: f64, t12046: f64, t12156: f64, t12451: f64, t1297: f64, t1390: f64, t193: f64, t533: f64, t571: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64) -> (f64, f64, f64) {
    let t12458 = t3698 * t1388;
    let t12461 = 1.0_f64 / t3700 / t570;
    let t12465 = t12451 * t1390 * t193 * t533 + 2.0_f64 * t12458 * t12461 * t193 * t533 + 3.0_f64 * t12012 * t1297 * t193 + 6.0_f64 * t12156 * t193 * t571 + t11976 - t11978 - t11980 - t11982 - t11984 + t12044 - t12046 - t9457 + t9476 + t9484 + t9780;
    (t12458, t12461, t12465)
}
