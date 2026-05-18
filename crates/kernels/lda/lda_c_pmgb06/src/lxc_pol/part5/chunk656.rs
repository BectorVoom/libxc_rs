//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 656/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk656<F: Float>(t5770: F, t5772: F, t348: F, t776: F, t110: F, t2217: F, t360: F, t2186: F, t947: F, t410: F, t2233: F, t365: F) -> (F, F, F, F, F, F, F, F) {
    let t5774 = F::new(5.87616) * t5770 * t5772;
    let t5783 = t348 * t776;
    let t5785 = F::new(1.9486833333333333) * t5783 * t5772;
    let t5787 = t360 * t110 * t2217;
    let t5788 = t2186 * t947;
    let t5790 = t410 * t776;
    let t5791 = t360 * t5790;
    let t5793 = t365 * t2233;
    (t5774, t5783, t5785, t5787, t5788, t5790, t5791, t5793)
}
