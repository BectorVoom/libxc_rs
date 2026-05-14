//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 758/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk758<F: Float>(t16739: F, t16742: F, t16745: F, t16748: F, t16751: F, t16756: F, t16760: F, t16922: F, t16925: F, t16928: F, t17349: F, t17454: F, t17459: F, t17472: F, t143: F, t160: F) -> (F, F) {
    let t17484 = -2.0 * t16739 + 4.0 / 3.0 * t16742 + t16745 / 27.0 - 2.0 / 27.0 * t16748 + 2.0 / 81.0 * t16751 + 2.0 / 3.0 * t16756 - t16760 / 9.0 + t17349 / 6.0 - t16922 / 3.0 + t16925 / 9.0 - 2.0 / 9.0 * t16928;
    let t17486 = t17454 + t17459 + t17472 + t17484;
    let t17488 = t143 * t17486 * t160;
    (t17486, t17488)
}
