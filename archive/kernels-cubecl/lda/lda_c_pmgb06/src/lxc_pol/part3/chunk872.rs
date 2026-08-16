//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 872/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk872<F: Float>(t3725: F, t971: F, t3758: F, t683: F, t696: F, t978: F, t3741: F, t957: F, t3738: F, t964: F, t246: F, t245: F, t286: F) -> (F, F, F, F, F) {
    let t8762 = t971 * t3725;
    let t8769 = F::cast_from(4.678578898107717_f64) * t696 * t978 * t3758 * t683;
    let t8771 = t3741 * t957;
    let t8774 = F::cast_from(6152.411314929844_f64) * t696 * t3738 * t964 * t8771;
    let t8775 = t246 * t246;
    let t8779 = F::cast_from(840.0_f64) * t245 / t8775 * t286;
    (t8762, t8769, t8771, t8774, t8779)
}
