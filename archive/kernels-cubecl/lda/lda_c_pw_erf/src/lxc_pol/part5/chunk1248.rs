//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1248/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1248<F: Float>(t4738: F, t6689: F, t6693: F, t17637: F, t1996: F, t3965: F, t18192: F, t595: F, t7470: F, t184: F, t811: F, t820: F) -> (F, F, F, F, F, F) {
    let t22385 = t4738 * t6689;
    let t22386 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t22385;
    let t22388 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4738 * t6693;
    let t22391 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t17637 * t1996;
    let t22392 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t18192;
    let t22394 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t7470 * t595;
    let t22396 = t811 * t820 * t184;
    (t22386, t22388, t22391, t22392, t22394, t22396)
}
