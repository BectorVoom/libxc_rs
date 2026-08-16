//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 910/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk910<F: Float>(t673: F, t680: F, t8698: F, t2372: F, t8656: F, t2354: F, t2698: F, t678: F, t2375: F, t2366: F, t56: F, t649: F, t691: F) -> (F, F, F, F, F) {
    let t8700 = t673 * t8698 * t680;
    let t8704 = t2372 * t8656 * t680;
    let t8708 = t2354 * t678 * t2698;
    let t8712 = t2375 * t678;
    let t8713 = t2372 * t2366 * t8712;
    let t8717 = t649 * t691 * t56;
    (t8700, t8704, t8708, t8713, t8717)
}
