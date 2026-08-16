//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2426/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426<F: Float>(t49379: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t42212: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> (F, F) {
    let t69118 = -t49379 - F::cast_from(0.516475e0_f64) * t68571 + F::cast_from(0.125034e1_f64) * t60192 - F::cast_from(0.83356000000000000002e0_f64) * t60194 - F::cast_from(0.41678e0_f64) * t60202 + F::cast_from(0.123954e2_f64) * t68577 - F::cast_from(0.929655e1_f64) * t68580 + F::cast_from(0.309885e1_f64) * t68583 + F::cast_from(0.309885e1_f64) * t68586 + F::cast_from(0.103295e1_f64) * t68589 - F::cast_from(0.34431666666666666667e0_f64) * t68592;
    let t69130 = F::cast_from(0.68863333333333333334e1_f64) * t68596 - F::cast_from(0.17215833333333333334e1_f64) * t68599 + F::cast_from(0.61977e1_f64) * t68602 - F::cast_from(0.17215833333333333333e1_f64) * t68605 - F::cast_from(0.929655e1_f64) * t68608 - F::cast_from(0.11577222222222222223e0_f64) * t60204 - F::cast_from(0.103295e1_f64) * t59663 + F::cast_from(0.34431666666666666666e0_f64) * t59665 + F::cast_from(0.51647499999999999999e0_f64) * t59680 + F::cast_from(0.13772666666666666667e1_f64) * t59688 - F::cast_from(0.68863333333333333332e0_f64) * t59694 + t42212;
    (t69118, t69130)
}
