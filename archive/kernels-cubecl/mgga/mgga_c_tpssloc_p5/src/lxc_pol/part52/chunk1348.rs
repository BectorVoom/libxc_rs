//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1348/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1348<F: Float>(t120568: F, t22674: F, t32697: F, t6897: F, t114253: F, t114225: F, t120551: F, t120552: F, t120553: F, t120556: F, t120561: F, t120566: F, t1375: F, t1842: F, t2015: F, t22656: F, t26224: F, t26225: F, t26347: F, t26471: F, t31189: F, t31216: F, t3887: F, t5210: F, t5354: F, t568: F, t7729: F, t8470: F) -> F {
    let t120569 = F::cast_from(0.82246703342411321825e-2_f64) * t120568;
    let t120576 = t6897 * t22674 * t32697;
    let t120577 = F::cast_from(0.82246703342411321825e-2_f64) * t120576;
    let t120579 = F::cast_from(0.38381794893125283518e-1_f64) * t114253;
    let t120582 = F::cast_from(2.0_f64) * t1375 * t1842 * t31216 * t3887 + F::cast_from(4.0_f64) * t1375 * t2015 * t26471 * t3887 - F::cast_from(12.0_f64) * t26224 * t26225 * t26347 + t5210 * t568 * t8470 + F::cast_from(4.0_f64) * t22656 * t7729 - t31189 * t5354 + t114225 - t120551 - t120552 + t120553 + t120556 - t120561 - t120566 + t120569 + t120577 + t120579;
    t120582
}
