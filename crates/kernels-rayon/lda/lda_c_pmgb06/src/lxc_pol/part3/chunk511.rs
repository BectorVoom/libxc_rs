//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 511/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk511(t2160: f64, t248: f64, t1067: f64, t1098: f64, t1103: f64, t2149: f64, t2152: f64, t2154: f64, t2156: f64, t2158: f64, t961: f64, t970: f64, t972: f64, t982: f64) -> f64 {
    let t2161 = t248 * t2160;
    let t2163 = -t1098 + t1103 - 0.5848223622634646_f64 * t2149 - 0.00018311447306006544_f64 * t2152 + t1067 - t961 + 4.0_f64 * t2154 - 4.0_f64 * t2156 + t248 * t2158 + t2161 - t970 - 0.5848223622634646_f64 * t972 + t982;
    t2163
}
