//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1096/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1096<F: Float>(t4612: F, t5211: F, t1983: F, t485: F, t5210: F, t5322: F, t5499: F, t806: F, t9836: F, t2007: F, t3220: F, t835: F, t9271: F) -> (F, F, F, F, F, F) {
    let t13196 = t5211 * t4612;
    let t13199 = t485 * t5210 * t1983;
    let t13201 = t5499 * t5322;
    let t13204 = t9836 * t806;
    let t13206 = t3220 * t2007;
    let t13211 = t9271 * t835;
    (t13196, t13199, t13201, t13204, t13206, t13211)
}
