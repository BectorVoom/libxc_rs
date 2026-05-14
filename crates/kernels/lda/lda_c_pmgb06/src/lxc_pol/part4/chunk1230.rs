//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1230/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1230<F: Float>(t17794: F, t17797: F, t17800: F, t17802: F, t17804: F, t17806: F, t17808: F, t17810: F, t17812: F, t17815: F, t17819: F, t17822: F, t17824: F, t17828: F, t18355: F, t18377: F, t224: F, t44: F) -> (F,) {
    let t18383 = -(t18355 / 2.0 + t18377 / 2.0) * t44 * t224 / 15.0 - t17794 - t17797 + t17800 - t17802 - t17804 - t17806 - t17808 + t17810 - t17812 - t17815 - t17819 + t17822 - t17824 - t17828;
    (t18383,)
}
