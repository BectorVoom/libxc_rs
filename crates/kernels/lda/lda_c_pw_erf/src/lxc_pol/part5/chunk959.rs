//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 959/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk959<F: Float>(t12557: F, t2151: F, t825: F, t571: F, t2176: F, t798: F, t519: F, t2171: F, t3784: F, t2076: F, t3660: F, t197: F, t4906: F) -> (F, F, F, F, F, F) {
    let t12558 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12557;
    let t12571 = t2151 * t825;
    let t12572 = t571 * t12571;
    let t12615 = t2176 * t798;
    let t12616 = t519 * t12615;
    let t12637 = t2171 * t3784;
    let t12638 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t12637;
    let t12684 = t2076 * t3660;
    let t12685 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12684;
    let t12695 = t4906 * t197;
    (t12558, t12572, t12616, t12638, t12685, t12695)
}
