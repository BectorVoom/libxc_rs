//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1067/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1067<F: Float>(t1: F, t2570: F, t1830: F, t453: F, t350: F, t7486: F, t7494: F, t1863: F, t5961: F, t36: F, t2381: F, t4667: F) -> (F, F, F, F, F, F, F) {
    let t19770 = t2570 * t1;
    let t19772 = t1830 * t453 * t19770;
    let t19774 = t350 * t7486;
    let t19776 = t350 * t7494;
    let t19778 = t1863 * t5961;
    let t19780 = t36 * t453 * t19778;
    let t19782 = t4667 * t2381;
    (t19770, t19772, t19774, t19776, t19778, t19780, t19782)
}
