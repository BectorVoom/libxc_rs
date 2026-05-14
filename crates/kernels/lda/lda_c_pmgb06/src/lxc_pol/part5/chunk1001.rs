//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1001/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1001<F: Float>(t132: F, t137: F, t153: F, t20715: F, t20734: F, t432: F, t7813: F, t10178: F, t7811: F, t17506: F, t6613: F, t802: F, t27: F, t545: F, t7704: F, t17544: F, t20684: F, t20689: F, t20692: F, t20694: F) -> (F, F, F, F, F, F) {
    let t20739 = t132 * t137 * (t20715 + t20734) * t153 / 30.0;
    let t20741 = t432 * t7813 / 5.0;
    let t20745 = t132 * t137 * t10178 * t7811 / 5.0;
    let t20746 = 2.0 / 15.0 * t17506;
    let t20747 = t802 * t6613;
    let t20748 = 2.0 / 15.0 * t20747;
    let t20750 = t7704 * t27 * t545;
    let t20753 = t20684 + t20689 + t20692 + t20694 + t20739 - t20741 - t20745 - t20746 - t20748 + 0.10821041362364843 * t20750 + 0.3246312408709453 * t17544;
    (t20739, t20741, t20745, t20746, t20748, t20753)
}
