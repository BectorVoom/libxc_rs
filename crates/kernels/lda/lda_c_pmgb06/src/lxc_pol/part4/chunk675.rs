//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 675/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk675<F: Float>(t3778: F, t667: F, t1007: F, t659: F, t260: F, t666: F, t993: F, t1010: F, t268: F, t265: F, t1011: F, t3758: F, t682: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3779 = t3778 * t667;
    let t3783 = F::cast_from(1.0_f64) / t1007 / t659;
    let t3784 = t260 * t3783;
    let t3785 = t993 * t666;
    let t3787 = F::cast_from(1.0_f64) / t1010 / t268;
    let t3788 = t3785 * t3787;
    let t3792 = F::cast_from(1.0_f64) / t1007 / t265;
    let t3793 = t260 * t3792;
    let t3794 = t3785 * t1011;
    let t3797 = t3758 * t682;
    (t3779, t3783, t3784, t3785, t3787, t3788, t3792, t3793, t3794, t3797)
}
