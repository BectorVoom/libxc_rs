//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 962/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk962<F: Float>(t11335: F, t5770: F, t8228: F, t5783: F, t2217: F, t360: F, t410: F, t365: F, t5740: F, t5772: F, t11334: F, t5756: F) -> (F, F, F, F, F, F) {
    let t11336 = F::new(8.769075) * t11335;
    let t11341 = t5770 * t8228;
    let t11343 = t5783 * t8228;
    let t11344 = F::new(2.923025) * t11343;
    let t11354 = t360 * t410 * t2217;
    let t11355 = F::new(2.0) * t11354;
    let t11357 = t365 * t5740 * t5772;
    let t11364 = t365 * t5756 * t11334;
    (t11336, t11341, t11344, t11355, t11357, t11364)
}
