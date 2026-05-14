//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1354/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1354<F: Float>(t105671: F, t105677: F, t105685: F, t105696: F, t105674: F, t105682: F, t105689: F, t105693: F, t105699: F, t95190: F, t95205: F, t105703: F, t105708: F, t105711: F, t105715: F, t105720: F, t105722: F, t105725: F, t105730: F, t105733: F, t95207: F, t95225: F, t95228: F) -> (F, F) {
    let t106062 = t105671 / 18.0;
    let t106064 = 4.0 / 9.0 * t105677;
    let t106067 = 2.0 / 9.0 * t105685;
    let t106070 = t105696 / 18.0;
    let t106072 = -t106062 - 2.0 / 9.0 * t105674 - t106064 - 4.0 / 9.0 * t95190 + 2.0 / 3.0 * t95205 + t105682 + t106067 - t105689 / 3.0 + t105693 / 12.0 - t106070 - 2.0 / 9.0 * t105699;
    let t106085 = -4.0 / 9.0 * t105703 + 2.0 / 3.0 * t105708 + t105711 / 27.0 + t105715 / 9.0 + t105720 / 3.0 - 4.0 / 9.0 * t105722 - 8.0 / 9.0 * t105725 - 2.0 / 27.0 * t95207 - t105730 / 6.0 - 4.0 / 27.0 * t105733 + 8.0 / 27.0 * t95225 + 2.0 / 27.0 * t95228;
    (t106072, t106085)
}
