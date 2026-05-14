//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1035/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1035<F: Float>(t21385: F, t348: F, t350: F, t7307: F, t365: F, t7310: F, t21358: F, t35: F, t64: F, t18625: F, t18628: F, t18630: F, t21376: F, t21379: F, t21382: F, t360: F, t8263: F) -> (F, F, F, F) {
    let t21386 = 2.923025 * t21385;
    let t21388 = t348 * t7307 * t350;
    let t21389 = 0.48717083333333333 * t21388;
    let t21391 = t365 * t7310 * t350;
    let t21394 = t35 * t64 * t21358;
    let t21397 = -3.0 * t18625 + 44.0712 * t18628 - 17.62848 * t18630 + 2.0 * t21376 + t21379 / 6.0 + 14.6904 * t21382 + t21386 + t21389 + 0.73452 * t21391 - t360 * t21394 / 2.0 + t8263;
    (t21386, t21389, t21394, t21397)
}
