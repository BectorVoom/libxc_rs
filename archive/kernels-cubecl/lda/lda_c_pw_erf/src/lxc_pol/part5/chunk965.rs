//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 965/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk965<F: Float>(t1124: F, t188: F, t1390: F, t2176: F, t10654: F, t1318: F, t2034: F, t1620: F, t838: F, t1931: F, t610: F, t2001: F) -> (F, F, F, F, F, F) {
    let t13172 = t1124 * t188;
    let t13202 = t2176 * t1390;
    let t13358 = t1318 * t10654 * t2034;
    let t13359 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13358;
    let t13377 = t838 * t1620;
    let t13379 = t1931 * t610;
    let t13380 = F::cast_from(8.0_f64) * t13379;
    let t13419 = t1318 * t10654 * t2001;
    (t13172, t13202, t13359, t13377, t13380, t13419)
}
