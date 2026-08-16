//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1236/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1236(t10895: f64, t10902: f64, t11589: f64, t1167: f64, t1200: f64, t123: f64, t125: f64, t14697: f64, t14700: f64, t14703: f64, t14706: f64, t14707: f64, t14710: f64, t14712: f64, t14723: f64, t14726: f64, t14741: f64, t14744: f64, t1808: f64, t199: f64, t2285: f64, t4209: f64, t4269: f64, t4464: f64, t566: f64, t868: f64, t912: f64) -> f64 {
    let t14746 = -t14697 - t14700 - t14703 - t14706 - t10902 - 1.279801625812305_f64 * t14707 + t14710 + 0.15917832887339686_f64 * t14712 - 0.031835665774679375_f64 * t123 * t125 * t11589 * t199 - 0.09550699732403813_f64 * t123 * t4464 * t566 + 0.15917832887339686_f64 * t14723 + 0.3183566577467937_f64 * t14726 - 0.09550699732403813_f64 * t123 * t2285 * t1200 - 0.031835665774679375_f64 * t123 * t912 * t4209 - 0.031835665774679375_f64 * t123 * t4269 * t868 - 0.09550699732403813_f64 * t123 * t1167 * t1808 + 0.15917832887339686_f64 * t14741 + 0.3183566577467937_f64 * t14744 + t10895;
    t14746
}
