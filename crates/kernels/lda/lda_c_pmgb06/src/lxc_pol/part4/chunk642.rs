//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 642/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk642<F: Float>(t1512: F, t436: F, t1517: F, t432: F, t1504: F, t486: F, t1554: F, t512: F, t161: F, t1499: F, t490: F, t1423: F, t1427: F) -> (F, F, F, F, F, F, F) {
    let t3149 = t1512 * t436;
    let t3151 = t432 * t1517;
    let t3153 = t486 * t1504;
    let t3155 = t1554 * t512;
    let t3156 = t161 * t3155;
    let t3158 = t1499 * t490;
    let t3165 = t1423 * t1427;
    (t3149, t3151, t3153, t3155, t3156, t3158, t3165)
}
