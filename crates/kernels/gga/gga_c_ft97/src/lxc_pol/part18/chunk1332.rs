//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1332/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1332<F: Float>(t12879: F, t1359: F, t28: F, t586: F, t5890: F, t27092: F, t376: F, t105368: F, t446: F, t9073: F, t105672: F, t105674: F, t105678: F, t105682: F, t105686: F, t105689: F, t96087: F, t96091: F) -> (F, F, F, F) {
    let t105693 = t5890 * t28 * t586 * t1359 * t12879;
    let t105696 = t5890 * t376 * t27092;
    let t105697 = t105696 / 6.0;
    let t105699 = t446 * t9073 * t105368;
    let t105701 = -t105672 - 2.0 / 3.0 * t105674 - t105678 - t96087 + t96091 + 3.0 * t105682 + t105686 - t105689 + t105693 / 4.0 - t105697 - 2.0 / 3.0 * t105699;
    (t105693, t105696, t105699, t105701)
}
