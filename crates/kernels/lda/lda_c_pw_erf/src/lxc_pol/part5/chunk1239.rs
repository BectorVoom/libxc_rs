//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1239/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1239<F: Float>(t348: F, t7639: F, t1326: F, t519: F, t7635: F, t1991: F, t2429: F, t34: F, t4829: F, t1318: F, t1319: F, t549: F, t7422: F) -> (F, F, F, F, F, F, F) {
    let t22277 = t7639 * t348;
    let t22280 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t519 * t1326 * t22277;
    let t22281 = t7635 * t348;
    let t22284 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t519 * t1991 * t22281;
    let t22285 = t2429 * t34;
    let t22288 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t519 * t4829 * t22285;
    let t22292 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t1319 * t7422 * t549;
    (t22277, t22280, t22281, t22284, t22285, t22288, t22292)
}
