//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1065/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1065<F: Float>(t486: F, t7618: F, t14348: F, t14350: F, t14357: F, t14359: F, t19736: F, t19738: F, t19739: F, t19740: F, t19741: F, t19742: F, t19746: F) -> (F, F) {
    let t19748 = t486 * t7618 / F::new(30.0);
    let t19751 = -t19736 - t19738 - t19739 - t19740 + t19741 + t19742 + t19746 + t19748 + t14348 + F::new(0.10063568466999305) * t14350 + t14357 + F::new(0.9738937226128359) * t14359;
    (t19748, t19751)
}
