//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1348/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1348(t120568: f64, t22674: f64, t32697: f64, t6897: f64, t114253: f64, t114225: f64, t120551: f64, t120552: f64, t120553: f64, t120556: f64, t120561: f64, t120566: f64, t1375: f64, t1842: f64, t2015: f64, t22656: f64, t26224: f64, t26225: f64, t26347: f64, t26471: f64, t31189: f64, t31216: f64, t3887: f64, t5210: f64, t5354: f64, t568: f64, t7729: f64, t8470: f64) -> f64 {
    let t120569 = 0.82246703342411321825e-2_f64 * t120568;
    let t120576 = t6897 * t22674 * t32697;
    let t120577 = 0.82246703342411321825e-2_f64 * t120576;
    let t120579 = 0.38381794893125283518e-1_f64 * t114253;
    let t120582 = 2.0_f64 * t1375 * t1842 * t31216 * t3887 + 4.0_f64 * t1375 * t2015 * t26471 * t3887 - 12.0_f64 * t26224 * t26225 * t26347 + t5210 * t568 * t8470 + 4.0_f64 * t22656 * t7729 - t31189 * t5354 + t114225 - t120551 - t120552 + t120553 + t120556 - t120561 - t120566 + t120569 + t120577 + t120579;
    t120582
}
