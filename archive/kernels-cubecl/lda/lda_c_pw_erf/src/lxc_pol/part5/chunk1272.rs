//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1272/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1272<F: Float>(t22832: F, t3802: F, t519: F, t7802: F, t571: F, t7727: F, t9678: F, t1318: F, t20972: F, t5269: F, t549: F, t18435: F) -> (F, F, F, F, F) {
    let t22833 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t22832;
    let t22835 = t519 * t3802 * t7802;
    let t22836 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t22835;
    let t22838 = t571 * t9678 * t7727;
    let t22839 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t22838;
    let t22843 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1318 * t5269 * t20972 * t549;
    let t22844 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t18435;
    (t22833, t22836, t22839, t22843, t22844)
}
