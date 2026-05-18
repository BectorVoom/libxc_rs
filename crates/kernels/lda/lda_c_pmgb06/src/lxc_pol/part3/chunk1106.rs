//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1106/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1106<F: Float>(t2002: F, t3012: F, t1420: F, t5203: F, t2007: F, t3177: F, t1511: F, t1980: F, t2012: F, t5171: F, t439: F, t805: F, t9373: F) -> (F, F, F, F, F, F) {
    let t13158 = F::new(2.0) / F::new(15.0) * t2002 * t3012;
    let t13160 = F::new(2.0) / F::new(5.0) * t1420 * t5203;
    let t13162 = t3177 * t2007 / F::new(15.0);
    let t13165 = F::new(2.0) / F::new(15.0) * t1511 * t1980 * t2012;
    let t13167 = t1420 * t5171 / F::new(15.0);
    let t13170 = t439 * t9373 * t805 / F::new(45.0);
    (t13158, t13160, t13162, t13165, t13167, t13170)
}
