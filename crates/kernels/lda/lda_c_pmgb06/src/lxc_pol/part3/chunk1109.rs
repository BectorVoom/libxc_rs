//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1109/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1109<F: Float>(t3384: F, t831: F, t1636: F, t1848: F, t2880: F, t4612: F, t5211: F, t1983: F, t485: F, t5210: F, t5322: F, t5499: F) -> (F, F, F, F, F, F) {
    let t13191 = t831 * t3384 / F::new(30.0);
    let t13192 = t1848 * t1636;
    let t13193 = F::new(2.0) / F::new(15.0) * t13192;
    let t13194 = t831 * t2880;
    let t13195 = F::new(2.0) / F::new(15.0) * t13194;
    let t13196 = t5211 * t4612;
    let t13197 = F::new(2.0) / F::new(9.0) * t13196;
    let t13199 = t485 * t5210 * t1983;
    let t13200 = F::new(2.0) / F::new(9.0) * t13199;
    let t13201 = t5499 * t5322;
    (t13191, t13193, t13195, t13197, t13200, t13201)
}
