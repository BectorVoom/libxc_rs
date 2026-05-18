//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 600/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk600<F: Float>(t1150: F, t1155: F, t1162: F, t1151: F, t1175: F, t1186: F, t1154: F, t251: F, t246: F, t1161: F, t272: F, t1156: F) -> (F, F, F, F, F, F, F, F) {
    let t4804 = t1150 * t1155;
    let t4806 = F::new(2.56) * t4804 * t1162;
    let t4807 = t1151 * t1175;
    let t4809 = t1151 * t1186;
    let t4812 = F::new(1.0) / t1154 / t251;
    let t4813 = t246 * t4812;
    let t4814 = t1161 * t1161;
    let t4815 = t272 * t4814;
    let t4817 = F::new(2.56) * t4813 * t4815;
    let t4818 = t1175 * t1161;
    let t4819 = t1156 * t4818;
    (t4804, t4806, t4807, t4809, t4813, t4814, t4817, t4819)
}
