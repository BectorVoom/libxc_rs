//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1269/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1269<F: Float>(t19694: F, t3200: F, t95848: F, t19698: F, t92808: F, t19750: F, t95911: F, t1020: F, t4792: F, t95664: F, t19149: F, t4994: F, t7718: F) -> (F, F, F, F, F) {
    let t100762 = t3200 * t95848 * t19694;
    let t100765 = t3200 * t92808 * t19698;
    let t100768 = t3200 * t95911 * t19750;
    let t100778 = t1020 * t95664 * t4792;
    let t100781 = t4994 * t7718 * t19149;
    (t100762, t100765, t100768, t100778, t100781)
}
