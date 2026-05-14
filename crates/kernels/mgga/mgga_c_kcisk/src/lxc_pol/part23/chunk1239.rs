//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1239/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1239<F: Float>(t9532: F, t9860: F, t32417: F, t32439: F, t33393: F, t33762: F, t33817: F, t33823: F, t33827: F, t33832: F, t33837: F, t9516: F, t9519: F, t9536: F, t9544: F, t9851: F, t9855: F) -> (F,) {
    let t33846 = t9860 * t9532;
    let t33848 = -0.30952962962962962963e-2 * t33393 + 0.17361111111111111111e-2 * t9536 * t33817 - 0.20104166666666666667e-2 * t32439 * t33762 + 0.20104166666666666667e-2 * t9516 * t33823 - 0.34722222222222222222e-2 * t9536 * t33827 - 0.10416666666666666667e-1 * t9536 * t33832 - 0.52083333333333333333e-2 * t9536 * t33837 + 0.20104166666666666667e-2 * t32417 * t9855 + 0.52083333333333333333e-2 * t9851 * t9544 + 0.52083333333333333333e-2 * t9851 * t9519 - 0.17361111111111111111e-2 * t33846;
    (t33848,)
}
