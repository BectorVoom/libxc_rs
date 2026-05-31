//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1227/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1227<F: Float>(t14030: F, t15727: F, t3974: F, t4818: F, t568: F, t7470: F, t515: F, t7466: F, t10427: F, t2146: F, t6195: F, t2188: F, t6198: F) -> (F, F, F, F, F, F) {
    let t22141 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3974 * t14030 * t4818 * t15727;
    let t22142 = t7470 * t568;
    let t22143 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t22142;
    let t22144 = t7466 * t515;
    let t22145 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t22144;
    let t22146 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t10427;
    let t22148 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2146 * t6195;
    let t22150 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6198 * t2188;
    (t22141, t22143, t22145, t22146, t22148, t22150)
}
