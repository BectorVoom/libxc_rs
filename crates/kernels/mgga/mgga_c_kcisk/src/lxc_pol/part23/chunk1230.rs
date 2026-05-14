//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1230/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1230<F: Float>(t33674: F, t33677: F, t33679: F, t33681: F, t33683: F, t33685: F, t33687: F, t33689: F, t33691: F, t33693: F, t33695: F, t33697: F, t33699: F, t33728: F, t1610: F, t9878: F) -> (F, F) {
    let t33742 = -0.26979166666666666667e-1 * t33674 + 0.625e-1 * t33677 + 0.20234375e-1 * t33679 + 0.1875e0 * t33681 + 0.20234375e-1 * t33683 + 0.10791666666666666667e0 * t33685 - 0.26979166666666666667e-1 * t33687 - 0.625e-1 * t33689 + 0.625e-1 * t33691 - 0.10791666666666666667e0 * t33693 - 0.20833333333333333333e-1 * t33695 - 0.625e-1 * t33697 + 0.25e0 * t33699;
    let t33743 = t33728 + t33742;
    let t33745 = t9878 * t1610;
    (t33743, t33745)
}
