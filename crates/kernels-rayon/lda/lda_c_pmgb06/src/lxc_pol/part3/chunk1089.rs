//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1089/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1089(t9805: f64, t9821: f64, t9826: f64, t12902: f64, t12907: f64, t12909: f64, t12911: f64, t12914: f64, t12916: f64, t12918: f64, t12920: f64, t12941: f64, t12962: f64, t224: f64, t44: f64) -> (f64, f64, f64, f64) {
    let t12968 = 4.0_f64 / 45.0_f64 * t9805;
    let t12969 = 4.0_f64 / 45.0_f64 * t9821;
    let t12970 = 4.0_f64 / 45.0_f64 * t9826;
    let t12971 = t12902 + t12907 + t12909 + t12911 + t12914 + t12916 + t12918 + t12920 - (t12941 / 2.0_f64 + t12962 / 2.0_f64) * t44 * t224 / 15.0_f64 - t12968 - t12969 - t12970;
    (t12968, t12969, t12970, t12971)
}
