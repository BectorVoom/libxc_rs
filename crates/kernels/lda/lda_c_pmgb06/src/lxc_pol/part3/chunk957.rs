//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 957/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk957<F: Float>(t9805: F, t9821: F, t9826: F, t12902: F, t12907: F, t12909: F, t12911: F, t12914: F, t12916: F, t12918: F, t12920: F, t12941: F, t12962: F, t224: F, t44: F, t9828: F) -> (F, F, F, F, F) {
    let t12968 = 4.0 / 45.0 * t9805;
    let t12969 = 4.0 / 45.0 * t9821;
    let t12970 = 4.0 / 45.0 * t9826;
    let t12971 = t12902 + t12907 + t12909 + t12911 + t12914 + t12916 + t12918 + t12920 - (t12941 / 2.0 + t12962 / 2.0) * t44 * t224 / 15.0 - t12968 - t12969 - t12970;
    let t12973 = 4.0 / 135.0 * t9828;
    (t12968, t12969, t12970, t12971, t12973)
}
