//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1354/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1354<F: Float>(t10777: F, t15116: F, t17787: F, t17790: F, t17794: F, t17797: F, t17800: F, t17802: F, t17804: F, t17806: F, t17808: F, t17810: F, t17812: F, t17815: F, t183: F, t188: F) -> F {
    let t17816 = t10777 + F::new(4.0) / F::new(3.0) * t15116 * t183 * t188 + F::new(8.0) / F::new(3.0) * t17787 + F::new(4.0) / F::new(3.0) * t17790 - t17794 - t17797 + t17800 - t17802 - t17804 - t17806 - t17808 + t17810 - t17812 - t17815;
    t17816
}
