//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1093/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1093<F: Float>(t97742: F, t97744: F, t97746: F, t97748: F, t97750: F, t97752: F, t97754: F, t97756: F, t97758: F, t97760: F, t97762: F, t97765: F, t97768: F, t97770: F, t97773: F, t97775: F, t97777: F, t97779: F) -> (F,) {
    let t97939 = t97742 / 144.0 + 2.0 / 9.0 * t97744 - t97746 / 12.0 - t97748 / 24.0 + t97750 / 128.0 + t97752 / 432.0 - t97754 / 64.0 - t97756 / 72.0 - t97758 / 24.0 + t97760 / 48.0 + t97762 / 48.0 - t97765 / 32.0 + t97768 / 48.0 - t97770 / 288.0 + t97773 / 12.0 - t97775 / 12.0 - t97777 / 48.0 - t97779 / 24.0;
    (t97939,)
}
