//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 702/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk702<F: Float>(t1423: F, t2501: F, t1447: F, t2497: F, t4837: F, t4845: F, t5045: F, t5047: F, t4786: F, t4788: F, t4792: F, t4794: F, t4807: F, t4809: F, t4812: F, t4814: F, t4950: F, t4970: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6423 = t1423 * t2501;
    let t6424 = F::new(4.0) / F::new(135.0) * t6423;
    let t6425 = t1447 * t2497;
    let t6426 = F::new(4.0) / F::new(135.0) * t6425;
    let t6427 = F::new(2.0) / F::new(135.0) * t4837;
    let t6428 = F::new(2.0) / F::new(135.0) * t4845;
    let t6429 = F::new(2.0) / F::new(135.0) * t5045;
    let t6430 = F::new(2.0) / F::new(135.0) * t5047;
    let t6431 = -t6424 - t6426 + t4786 + t4788 + t4792 + t4794 + t4807 + t4809 + t4812 + t4814 + t6427 + t6428 - t4950 - t4970 - t6429 - t6430;
    (t6423, t6424, t6425, t6426, t6427, t6428, t6429, t6430, t6431)
}
