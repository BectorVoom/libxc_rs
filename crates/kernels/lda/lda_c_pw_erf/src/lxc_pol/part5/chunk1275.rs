//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1275/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1275<F: Float>(t10379: F, t571: F, t593: F, t7408: F, t14257: F, t21811: F, t15123: F, t15125: F, t15138: F, t15140: F, t22857: F, t22859: F, t22860: F, t22861: F, t22862: F, t22863: F, t22868: F) -> (F, F, F) {
    let t22872 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t571 * t10379 * t7408 * t593;
    let t22875 = F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t571 * t14257 * t21811;
    let t22876 = t22857 - t22859 + t22860 + t22861 - t22862 + t22863 + t15123 + F::cast_from(0.547_f64) * t15125 + t15138 + t15140 + t22868 - t22872 + t22875;
    (t22872, t22875, t22876)
}
