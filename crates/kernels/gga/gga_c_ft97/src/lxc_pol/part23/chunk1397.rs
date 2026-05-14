//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1397/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1397<F: Float>(t126697: F, t126701: F, t126705: F, t126709: F, t126712: F, t126715: F, t126718: F, t126723: F, t126725: F, t126727: F, t126731: F, t114282: F, t126736: F, t126740: F, t126744: F, t126749: F, t126753: F, t126757: F, t126761: F, t126765: F, t126769: F, t126773: F, t126776: F) -> (F, F) {
    let t128162 = -t126697 / 3.0 - t126701 / 3.0 - t126705 / 3.0 + 5.0 / 16.0 * t126709 + 4.0 / 3.0 * t126712 + 8.0 / 9.0 * t126715 - 8.0 / 27.0 * t126718 + t126723 / 2.0 + t126725 / 27.0 + t126727 / 9.0 - t126731 / 3.0;
    let t128173 = -t126736 / 6.0 - t126740 - t126744 / 9.0 + t126749 / 4.0 - t126753 / 4.0 + 4.0 * t126757 - 2.0 * t126761 + t126765 / 9.0 - t126769 / 4.0 - t126773 / 36.0 + t114282 + 4.0 / 9.0 * t126776;
    (t128162, t128173)
}
