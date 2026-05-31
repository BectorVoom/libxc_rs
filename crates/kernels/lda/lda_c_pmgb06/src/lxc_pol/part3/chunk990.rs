//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 990/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk990<F: Float>(t9110: F, t9234: F, t3005: F, t831: F, t9237: F, t9239: F, t9242: F, t9259: F, t9267: F, t9269: F, t9272: F, t9274: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11775 = F::cast_from(16.0_f64) / F::cast_from(243.0_f64) * t9110;
    let t11776 = t9234 / F::cast_from(45.0_f64);
    let t11777 = t831 * t3005;
    let t11778 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t11777;
    let t11779 = t9237 / F::cast_from(45.0_f64);
    let t11780 = t9239 / F::cast_from(45.0_f64);
    let t11781 = t9242 / F::cast_from(45.0_f64);
    let t11782 = t9259 / F::cast_from(45.0_f64);
    let t11783 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t9267;
    let t11784 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t9269;
    let t11785 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t9272;
    let t11786 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9274;
    (t11775, t11776, t11778, t11779, t11780, t11781, t11782, t11783, t11784, t11785, t11786)
}
