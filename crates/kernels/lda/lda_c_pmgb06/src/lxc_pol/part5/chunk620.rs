//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 620/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk620<F: Float>(t2210: F, t348: F, t350: F, t1238: F, t773: F, t955: F, t110: F, t2221: F, t360: F, t2226: F, t947: F, t2236: F, t377: F, t1295: F, t783: F, t5790: F, t69: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5799 = t348 * t2210;
    let t5801 = 0.9743416666666667 * t5799 * t350;
    let t5802 = t1238 * t773;
    let t5803 = t5802 * t955;
    let t5806 = t110 * t2221;
    let t5808 = t360 * t5806 / 3.0;
    let t5813 = t2226 * t947;
    let t5831 = t2236 * t377;
    let t5834 = t783 * t1295;
    let t5852 = t69 * t5790;
    (t5799, t5801, t5802, t5803, t5806, t5808, t5813, t5831, t5834, t5852)
}
