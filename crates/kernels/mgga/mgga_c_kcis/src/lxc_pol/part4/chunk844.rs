//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 844/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk844<F: Float>(t656: F, t8590: F, t4620: F, t4714: F, t8594: F, t8596: F, t8598: F, t8691: F, t8693: F, t673: F, t680: F, t2372: F, t8656: F, t2354: F, t2698: F, t678: F) -> (F, F, F, F, F) {
    let t8695 = t656 * t8590;
    let t8698 = -0.34523333333333333333e1 * t8594 + 0.23015555555555555556e1 * t8596 - 0.26851481481481481482e1 * t8598 - 0.93932222222222222223e0 * t4620 + 0.73355e-1 * t8691 - 0.14671e0 * t8693 - 0.17116166666666666667e0 * t8695 - 0.36793333333333333333e0 * t4714;
    let t8700 = t673 * t8698 * t680;
    let t8704 = t2372 * t8656 * t680;
    let t8708 = t2354 * t678 * t2698;
    (t8695, t8698, t8700, t8704, t8708)
}
