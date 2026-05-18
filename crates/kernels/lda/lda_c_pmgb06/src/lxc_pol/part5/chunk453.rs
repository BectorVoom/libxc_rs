//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 453/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk453<F: Float>(t2209: F, t56: F, t38: F, t110: F, t776: F, t360: F, t342: F, t780: F, t64: F, t35: F, t365: F, t350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2210 = t56 * t2209;
    let t2212 = F::new(2.923025) * t38 * t2210;
    let t2214 = t110 * t776;
    let t2215 = t360 * t2214;
    let t2217 = t780 * t342;
    let t2221 = t64 * t2209;
    let t2222 = t35 * t2221;
    let t2226 = t365 * t780;
    let t2227 = t2226 * t350;
    (t2210, t2212, t2214, t2215, t2217, t2221, t2222, t2226, t2227)
}
