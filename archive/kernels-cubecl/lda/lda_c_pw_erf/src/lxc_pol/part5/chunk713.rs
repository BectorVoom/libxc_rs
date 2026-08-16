//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 713/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk713<F: Float>(t2017: F, t6379: F, t571: F, t2334: F, t3589: F, t352: F, t4776: F, t1943: F, t34: F, t4868: F, t2027: F, t4738: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6380 = t2017 * t6379;
    let t6382 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t571 * t6380;
    let t6383 = t3589 * t2334;
    let t6384 = t6383 * t352;
    let t6385 = t4776 * t6384;
    let t6387 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t571 * t6385;
    let t6388 = t1943 * t34;
    let t6389 = t4868 * t6388;
    let t6391 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t571 * t6389;
    let t6393 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4738 * t2027;
    (t6380, t6382, t6383, t6384, t6385, t6387, t6388, t6389, t6391, t6393)
}
