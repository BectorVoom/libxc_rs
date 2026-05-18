//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 997/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk997<F: Float>(t1992: F, t3459: F, t493: F, t851: F, t9636: F, t2007: F, t3213: F, t131: F, t1767: F, t129: F, t2012: F, t10318: F, t806: F) -> (F, F, F, F, F) {
    let t11859 = F::new(4.0) / F::new(5.0) * t493 * t1992 * t9636 * t851 * t3459;
    let t11860 = t3213 * t2007;
    let t11861 = F::new(2.0) / F::new(135.0) * t11860;
    let t11862 = t131 * t1767;
    let t11864 = t129 * t11862 * t2012;
    let t11865 = F::new(32.0) / F::new(135.0) * t11864;
    let t11866 = t10318 * t806;
    (t11859, t11861, t11862, t11865, t11866)
}
