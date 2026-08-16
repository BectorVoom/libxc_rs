//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 966/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk966<F: Float>(t13419: F, t4906: F, t529: F, t1124: F, t1458: F, t197: F, t4900: F, t581: F, t1484: F, t219: F, t2146: F, t3763: F) -> (F, F, F, F, F, F) {
    let t13420 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13419;
    let t13432 = t4906 * t529;
    let t13440 = t1124 * t1458 * t197;
    let t13444 = t4900 * t581;
    let t13455 = t1124 * t1484 * t219;
    let t13478 = t2146 * t3763;
    (t13420, t13432, t13440, t13444, t13455, t13478)
}
