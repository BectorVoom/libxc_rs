//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 968/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk968<F: Float>(t1869: F, t8337: F, t1830: F, t810: F, t4641: F, t4656: F, t350: F, t4673: F, t4669: F, t4660: F, t4646: F, t4664: F, t1865: F, t947: F, t1860: F, t4651: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12329 = t8337 * t1869;
    let t12337 = t1830 * t810;
    let t12354 = t4641 * t4656;
    let t12356 = t350 * t4673;
    let t12358 = t4641 * t4669;
    let t12360 = t350 * t4660;
    let t12362 = t350 * t4646;
    let t12364 = t350 * t4664;
    let t12366 = t947 * t1865;
    let t12368 = t947 * t1860;
    let t12393 = t350 * t4651;
    (t12329, t12337, t12354, t12356, t12358, t12360, t12362, t12364, t12366, t12368, t12393)
}
