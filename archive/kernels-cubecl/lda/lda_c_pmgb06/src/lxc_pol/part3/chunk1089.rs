//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1089/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1089<F: Float>(t9805: F, t9821: F, t9826: F, t12902: F, t12907: F, t12909: F, t12911: F, t12914: F, t12916: F, t12918: F, t12920: F, t12941: F, t12962: F, t224: F, t44: F) -> (F, F, F, F) {
    let t12968 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9805;
    let t12969 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9821;
    let t12970 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9826;
    let t12971 = t12902 + t12907 + t12909 + t12911 + t12914 + t12916 + t12918 + t12920 - (t12941 / F::cast_from(2.0_f64) + t12962 / F::cast_from(2.0_f64)) * t44 * t224 / F::cast_from(15.0_f64) - t12968 - t12969 - t12970;
    (t12968, t12969, t12970, t12971)
}
