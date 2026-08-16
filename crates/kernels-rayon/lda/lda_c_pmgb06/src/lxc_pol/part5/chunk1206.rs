//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1206/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1206(t11142: f64, t11147: f64, t11150: f64, t11152: f64, t11156: f64, t11157: f64, t11161: f64, t11162: f64, t11165: f64, t15026: f64, t15028: f64, t15030: f64, t8814: f64, t8822: f64, t8830: f64, t8834: f64) -> f64 {
    let t21812 = 10.526802520742363_f64 * t11142 - 155.84273195113317_f64 * t11147 + t11150 + t11152 - 12.0_f64 * t15026 - 12.0_f64 * t15028 - 24.0_f64 * t15030 + t8814 + t8822 + t8830 - t8834 + t11156 - 0.0017090684152272775_f64 * t11157 + t11161 + 311.68546390226635_f64 * t11162 - t11165;
    t21812
}
