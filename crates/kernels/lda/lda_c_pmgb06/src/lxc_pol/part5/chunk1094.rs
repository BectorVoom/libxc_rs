//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1094/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1094<F: Float>(t123: F, t199: F, t315: F, t7364: F, t395: F, t7425: F, t10895: F, t10902: F, t14700: F, t14703: F, t14706: F, t14707: F, t14710: F, t18988: F, t18995: F, t18998: F, t19004: F, t19007: F, t19017: F, t19020: F, t19031: F, t22088: F, t305: F, t726: F, t7375: F) -> (F,) {
    let t22220 = t123 * t315 * t7364 * t199;
    let t22233 = t395 * t7425;
    let t22236 = -t14700 - t14703 - t14706 - 0.031835665774679375 * t123 * t305 * t22088 + 0.053059442957798957 * t22220 - 0.031835665774679375 * t123 * t726 * t7375 - t10902 - 0.8489510873247833 * t18988 + 0.15917832887339686 * t18995 + 0.15917832887339686 * t18998 + 0.3183566577467937 * t19004 + 0.3183566577467937 * t19007 - 0.42447554366239165 * t19017 - 0.42447554366239165 * t19020 - 3.839404877436915 * t14707 + t14710 - 0.10665013548435875 * t22233 + 0.15917832887339686 * t19031 + t10895;
    (t22236,)
}
