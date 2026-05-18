//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 913/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk913<F: Float>(t1837: F, t831: F, t6612: F, t6615: F, t6618: F, t6620: F, t6623: F, t6625: F, t6628: F, t6632: F, t6635: F, t6641: F, t6645: F, t6648: F, t6650: F, t6652: F) -> (F, F) {
    let t6654 = t831 * t1837 / F::new(15.0);
    let t6655 = -t6612 - t6615 - t6618 - t6620 - t6623 - t6625 + t6628 + t6632 - t6635 + t6641 + t6645 - t6648 + t6650 + t6652 + t6654;
    (t6654, t6655)
}
