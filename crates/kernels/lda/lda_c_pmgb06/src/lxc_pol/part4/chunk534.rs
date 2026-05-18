//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 534/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk534<F: Float>(t2221: F, t35: F, t365: F, t780: F, t350: F, t1282: F, t769: F) -> (F, F, F, F) {
    let t2222 = t35 * t2221;
    let t2226 = t365 * t780;
    let t2227 = t2226 * t350;
    let t2229 = t1282 * t769;
    (t2222, t2226, t2227, t2229)
}
