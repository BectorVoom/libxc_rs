//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1272/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1272(t22832: f64, t3802: f64, t519: f64, t7802: f64, t571: f64, t7727: f64, t9678: f64, t1318: f64, t20972: f64, t5269: f64, t549: f64, t18435: f64) -> (f64, f64, f64, f64, f64) {
    let t22833 = 16.0_f64 / 45.0_f64 * t22832;
    let t22835 = t519 * t3802 * t7802;
    let t22836 = 8.0_f64 / 45.0_f64 * t22835;
    let t22838 = t571 * t9678 * t7727;
    let t22839 = 16.0_f64 / 45.0_f64 * t22838;
    let t22843 = 8.0_f64 / 5.0_f64 * t1318 * t5269 * t20972 * t549;
    let t22844 = 32.0_f64 / 45.0_f64 * t18435;
    (t22833, t22836, t22839, t22843, t22844)
}
