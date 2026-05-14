//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 981/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk981<F: Float>(t5322: F, t5499: F, t806: F, t9836: F, t2007: F, t3220: F, t835: F, t9271: F, t1977: F, t3226: F, t1447: F, t4605: F, t2012: F, t431: F, t5210: F, t1423: F, t5171: F) -> (F, F, F, F, F, F, F, F) {
    let t13201 = t5499 * t5322;
    let t13204 = t9836 * t806;
    let t13206 = t3220 * t2007;
    let t13211 = t9271 * t835;
    let t13213 = t3226 * t1977;
    let t13215 = t1447 * t4605;
    let t13218 = t431 * t5210 * t2012;
    let t13220 = t1423 * t5171;
    (t13201, t13204, t13206, t13211, t13213, t13215, t13218, t13220)
}
