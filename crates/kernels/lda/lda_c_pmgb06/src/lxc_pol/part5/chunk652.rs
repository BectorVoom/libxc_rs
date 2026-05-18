//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 652/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk652<F: Float>(t1179: F, t794: F, t419: F, t421: F, t1798: F, t409: F, t1186: F, t2329: F, t1193: F, t1354: F, t4429: F, t118: F, t2174: F) -> (F, F, F, F, F, F, F) {
    let t5613 = t1179 * t794;
    let t5615 = t5613 * t419 * t421;
    let t5617 = t409 * t1798;
    let t5620 = F::new(0.003950778065781896) * t5617 * t419 * t421;
    let t5622 = t2329 * t1186 * t421;
    let t5625 = t4429 * t1193 * t1354;
    let t5627 = t2174 * t118;
    (t5613, t5615, t5617, t5620, t5622, t5625, t5627)
}
