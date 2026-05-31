//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1215/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1215<F: Float>(t21927: F, t1318: F, t1466: F, t16907: F, t811: F, t34: F, t4892: F, t6188: F, t4753: F, t7570: F, t3416: F, t3899: F, t7596: F) -> (F, F, F, F, F, F) {
    let t21928 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t21927;
    let t21932 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1318 * t1466 * t16907 * t811;
    let t21936 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1318 * t4892 * t6188 * t34;
    let t21938 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4753 * t7570;
    let t21940 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3416 * t7570;
    let t21942 = t1318 * t3899 * t7596;
    (t21928, t21932, t21936, t21938, t21940, t21942)
}
