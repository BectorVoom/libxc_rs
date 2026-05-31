//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2925/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925<F: Float>(t77798: F, t916: F, t23510: F, t698: F, t23507: F, t141: F, t77533: F, t930: F, t77537: F, t77541: F, t77545: F, t52127: F, t52128: F, t63447: F, t63453: F, t63459: F) -> (F, F, F, F, F, F, F, F) {
    let t77802 = t916 * t77798;
    let t77804 = t698 * t23510;
    let t77806 = t698 * t23507;
    let t77810 = t141 * t930 * t77533;
    let t77813 = t141 * t930 * t77537;
    let t77816 = t141 * t930 * t77541;
    let t77819 = t141 * t930 * t77545;
    let t77824 = F::cast_from(0.258925e1_f64) * t77802 - F::cast_from(0.33114e0_f64) * t77804 + F::cast_from(0.5519e-1_f64) * t77806 - t52127 + F::cast_from(0.73586666666666666667e0_f64) * t52128 + F::cast_from(0.198684e1_f64) * t77810 - F::cast_from(0.149013e1_f64) * t77813 + F::cast_from(0.49671e0_f64) * t77816 + F::cast_from(0.49671e0_f64) * t77819 + F::cast_from(0.30192500000000000001e0_f64) * t63447 - F::cast_from(0.26837777777777777777e0_f64) * t63453 + F::cast_from(0.80513333333333333334e0_f64) * t63459;
    (t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824)
}
