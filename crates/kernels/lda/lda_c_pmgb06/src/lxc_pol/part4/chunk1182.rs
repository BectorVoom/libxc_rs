//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1182/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1182<F: Float>(t14015: F, t14465: F, t14467: F, t14469: F, t14471: F, t17858: F, t17859: F, t17861: F, t17863: F, t17869: F, t17871: F, t17873: F, t17876: F, t17878: F, t17879: F, t14017: F) -> (F, F, F) {
    let t17880 = 4.0 / 135.0 * t14015;
    let t17881 = -t17858 + 8.0 / 3.0 * t17859 + 8.0 / 3.0 * t17861 + t17863 + 8.0 * t14465 + 0.002206740740740741 * t14467 + 8.0 / 3.0 * t14469 + 32.0 / 3.0 * t14471 - t17869 - t17871 + t17873 - t17876 - t17878 - t17879 + t17880;
    let t17884 = 4.0 / 45.0 * t14017;
    (t17880, t17881, t17884)
}
