//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1095/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1095(t12535: f64, t441: f64, t5075: f64, t13021: f64, t5094: f64, t12683: f64, t5082: f64, t5087: f64, t13005: f64, t13009: f64, t13012: f64, t13015: f64, t13018: f64, t13024: f64, t13030: f64, t13034: f64, t13038: f64, t13041: f64) -> (f64, f64, f64) {
    let t13043 = t5075 * t12535 * t441;
    let t13046 = 8.0_f64 / 15.0_f64 * t13043 * t5094 * t13021;
    let t13047 = t12683 * t5082;
    let t13049 = 2.0_f64 / 9.0_f64 * t13047 * t5087;
    let t13050 = t13005 + t13009 + t13012 - t13015 + t13018 - t13024 + t13030 - t13034 - t13038 - t13041 + t13046 - t13049;
    (t13046, t13049, t13050)
}
