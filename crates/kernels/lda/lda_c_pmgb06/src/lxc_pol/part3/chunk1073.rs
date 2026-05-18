//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1073/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1073<F: Float>(t1069: F, t1438: F, t2064: F, t2960: F, t439: F, t1908: F, t3213: F, t12719: F, t12724: F, t12726: F, t12728: F, t12730: F, t12733: F, t12737: F, t12741: F, t12743: F, t12746: F) -> (F, F, F) {
    let t12751 = t439 * t2960 * t2064 * t1438 * t1069 / F::new(9.0);
    let t12752 = t3213 * t1908;
    let t12753 = F::new(2.0) / F::new(135.0) * t12752;
    let t12754 = -t12719 - t12724 - t12726 - t12728 - t12730 - t12733 - t12737 - t12741 - t12743 - t12746 - t12751 + t12753;
    (t12751, t12753, t12754)
}
