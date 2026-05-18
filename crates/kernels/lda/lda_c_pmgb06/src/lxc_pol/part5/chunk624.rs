//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 624/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk624<F: Float>(t2061: F, t4913: F, t2057: F, t405: F, t2054: F, t1554: F, t843: F, t161: F, t1555: F, t831: F, t1548: F, t802: F) -> (F, F, F, F, F, F, F) {
    let t5006 = t4913 * t2061;
    let t5032 = F::new(0.017777777777777778) * t405 * t2057;
    let t5034 = F::new(0.002962962962962963) * t405 * t2054;
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5047 = t831 * t1555;
    let t5049 = t802 * t1548;
    (t5006, t5032, t5034, t5044, t5045, t5047, t5049)
}
