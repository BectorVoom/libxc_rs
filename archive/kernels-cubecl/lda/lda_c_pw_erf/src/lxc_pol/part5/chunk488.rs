//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 488/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk488<F: Float>(t1308: F, t2388: F, t571: F, t2005: F, t739: F, t1326: F) -> (F, F, F, F) {
    let t2389 = t1308 * t2388;
    let t2391 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t571 * t2389;
    let t2392 = t2005 * t739;
    let t2393 = t1326 * t2392;
    (t2389, t2391, t2392, t2393)
}
