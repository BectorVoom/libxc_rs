//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 541/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk541<F: Float>(t115: F, t2789: F, t117: F, t84: F, t1347: F, t399: F, t391: F, t395: F, t1: F, t247: F) -> (F, F, F, F, F, F, F) {
    let t2790 = t2789 * t115;
    let t2791 = t2790 * t117;
    let t2793 = F::new(0.031505407223141116) * t84 * t2791;
    let t2794 = t399 * t1347;
    let t2797 = F::new(0.09451622166942335) * t391 * t1347;
    let t2798 = F::new(12.0) * t395;
    let t2799 = t1 * t247;
    (t2790, t2791, t2793, t2794, t2797, t2798, t2799)
}
