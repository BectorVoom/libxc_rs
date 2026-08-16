//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1085/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1085(t1925: f64, t3198: f64, t9774: f64, t1179: f64, t161: f64, t4840: f64, t495: f64, t9777: f64, t12879: f64, t12881: f64, t12883: f64, t12885: f64, t12887: f64, t12889: f64, t12892: f64) -> (f64, f64, f64, f64, f64) {
    let t12894 = t3198 * t1925 / 15.0_f64;
    let t12895 = t9774 / 45.0_f64;
    let t12898 = t161 * t1179 * t495 * t4840;
    let t12899 = 8.0_f64 / 45.0_f64 * t12898;
    let t12900 = 2.0_f64 / 15.0_f64 * t9777;
    let t12901 = -t12879 - t12881 - t12883 - t12885 + t12887 - t12889 - t12892 - t12894 + t12895 + t12899 + t12900;
    (t12894, t12895, t12899, t12900, t12901)
}
