//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1128/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1128<F: Float>(t34: F, t6378: F, t4868: F, t571: F, t2100: F, t2443: F, t15764: F, t14979: F, t14980: F, t20897: F, t20898: F, t20899: F, t20901: F, t20903: F, t20905: F, t20910: F, t9250: F) -> (F, F, F, F, F) {
    let t20911 = t6378 * t34;
    let t20914 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t571 * t4868 * t20911;
    let t20916 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t2443 * t2100;
    let t20917 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t15764;
    let t20919 = t20897 - t9250 + t20898 + t20899 - t20901 - t20903 + t20905 - t20910 + t20914 - t20916 + t20917 + t14979 + F::cast_from(0.299209_f64) * t14980;
    (t20911, t20914, t20916, t20917, t20919)
}
