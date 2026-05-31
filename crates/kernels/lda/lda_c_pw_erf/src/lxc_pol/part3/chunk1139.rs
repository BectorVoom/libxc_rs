//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1139/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1139<F: Float>(t4753: F, t4760: F, t3416: F, t34: F, t3604: F, t951: F, t4868: F, t571: F, t2018: F, t3742: F, t5285: F, t9678: F) -> (F, F, F, F, F, F) {
    let t13340 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4753 * t4760;
    let t13342 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3416 * t4760;
    let t13344 = t3604 * t34 * t951;
    let t13347 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t571 * t4868 * t13344;
    let t13349 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3742 * t2018;
    let t13351 = t571 * t9678 * t5285;
    (t13340, t13342, t13344, t13347, t13349, t13351)
}
