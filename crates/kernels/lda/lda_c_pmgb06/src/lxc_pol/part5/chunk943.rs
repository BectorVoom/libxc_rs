//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 943/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk943<F: Float>(t1: F, t6145: F, t1525: F, t1830: F, t1858: F, t5961: F, t36: F, t2381: F, t4654: F, t332: F, t7481: F, t453: F, t2570: F, t350: F, t7486: F, t7494: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19754 = t6145 * t1;
    let t19756 = t1830 * t1525 * t19754;
    let t19758 = t1858 * t5961;
    let t19760 = t36 * t1525 * t19758;
    let t19762 = t4654 * t2381;
    let t19764 = t1830 * t1525 * t19762;
    let t19766 = t7481 * t332;
    let t19768 = t36 * t453 * t19766;
    let t19770 = t2570 * t1;
    let t19772 = t1830 * t453 * t19770;
    let t19774 = t350 * t7486;
    let t19776 = t350 * t7494;
    (t19754, t19756, t19758, t19760, t19762, t19764, t19766, t19768, t19770, t19772, t19774, t19776)
}
