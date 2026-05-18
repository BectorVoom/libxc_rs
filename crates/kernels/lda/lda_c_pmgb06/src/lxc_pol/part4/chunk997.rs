//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 997/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk997<F: Float>(t3724: F, t3758: F, t696: F, t963: F, t3729: F, t971: F, t3725: F, t683: F, t978: F, t3741: F, t957: F, t3738: F, t964: F) -> (F, F, F, F, F, F) {
    let t8759 = F::new(69.26343642272586) * t696 * t963 * t3758 * t3724;
    let t8760 = t971 * t3729;
    let t8762 = t971 * t3725;
    let t8769 = F::new(4.678578898107717) * t696 * t978 * t3758 * t683;
    let t8771 = t3741 * t957;
    let t8774 = F::new(6152.411314929844) * t696 * t3738 * t964 * t8771;
    (t8759, t8760, t8762, t8769, t8771, t8774)
}
