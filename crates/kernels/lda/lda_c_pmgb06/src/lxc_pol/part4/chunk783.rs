//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 783/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk783<F: Float>(t3505: F, t3513: F, t3515: F, t3517: F, t3521: F, t3523: F, t3525: F, t360: F, t5805: F, t5808: F, t5810: F, t5813: F, t5827: F, t2236: F, t377: F, t1295: F, t783: F) -> (F, F, F) {
    let t5829 = t5805 + t5808 - t360 * t5810 / 2.0 - 0.97936 * t5813 - t3505 + t3513 - t3515 - t3517 - t3521 - t3523 + t3525 + t5827;
    let t5831 = t2236 * t377;
    let t5834 = t783 * t1295;
    (t5829, t5831, t5834)
}
