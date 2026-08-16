//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1264/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1264(t123: f64, t199: f64, t315: f64, t7364: f64, t395: f64, t7425: f64, t10895: f64, t10902: f64, t14700: f64, t14703: f64, t14706: f64, t14707: f64, t14710: f64, t18988: f64, t18995: f64, t18998: f64, t19004: f64, t19007: f64, t19017: f64, t19020: f64, t19031: f64, t22088: f64, t305: f64, t726: f64, t7375: f64) -> f64 {
    let t22220 = t123 * t315 * t7364 * t199;
    let t22233 = t395 * t7425;
    let t22236 = -t14700 - t14703 - t14706 - 0.031835665774679375_f64 * t123 * t305 * t22088 + 0.053059442957798957_f64 * t22220 - 0.031835665774679375_f64 * t123 * t726 * t7375 - t10902 - 0.8489510873247833_f64 * t18988 + 0.15917832887339686_f64 * t18995 + 0.15917832887339686_f64 * t18998 + 0.3183566577467937_f64 * t19004 + 0.3183566577467937_f64 * t19007 - 0.42447554366239165_f64 * t19017 - 0.42447554366239165_f64 * t19020 - 3.839404877436915_f64 * t14707 + t14710 - 0.10665013548435875_f64 * t22233 + 0.15917832887339686_f64 * t19031 + t10895;
    t22236
}
