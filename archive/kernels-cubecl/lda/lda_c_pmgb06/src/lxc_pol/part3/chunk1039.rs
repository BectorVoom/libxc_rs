//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1039/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1039<F: Float>(t4641: F, t4656: F, t350: F, t4673: F, t4669: F, t4660: F, t4646: F, t4664: F, t1865: F, t947: F, t1860: F, t12146: F, t1525: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12354 = t4641 * t4656;
    let t12356 = t350 * t4673;
    let t12358 = t4641 * t4669;
    let t12360 = t350 * t4660;
    let t12362 = t350 * t4646;
    let t12364 = t350 * t4664;
    let t12366 = t947 * t1865;
    let t12368 = t947 * t1860;
    let t12369 = F::cast_from(0.0016792592592592592_f64) * t12368;
    let t12371 = t36 * t1525 * t12146;
    (t12354, t12356, t12358, t12360, t12362, t12364, t12366, t12368, t12369, t12371)
}
