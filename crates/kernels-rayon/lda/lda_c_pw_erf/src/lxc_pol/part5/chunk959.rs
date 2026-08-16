//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 959/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk959(t12557: f64, t2151: f64, t825: f64, t571: f64, t2176: f64, t798: f64, t519: f64, t2171: f64, t3784: f64, t2076: f64, t3660: f64, t197: f64, t4906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12558 = 8.0_f64 / 45.0_f64 * t12557;
    let t12571 = t2151 * t825;
    let t12572 = t571 * t12571;
    let t12615 = t2176 * t798;
    let t12616 = t519 * t12615;
    let t12637 = t2171 * t3784;
    let t12638 = 8.0_f64 / 135.0_f64 * t12637;
    let t12684 = t2076 * t3660;
    let t12685 = 8.0_f64 / 45.0_f64 * t12684;
    let t12695 = t4906 * t197;
    (t12558, t12572, t12616, t12638, t12685, t12695)
}
