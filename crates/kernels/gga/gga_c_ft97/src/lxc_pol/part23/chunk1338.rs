//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1338/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1338<F: Float>(t126697: F, t126701: F, t126705: F, t126709: F, t126712: F, t126715: F, t126718: F, t126723: F, t126726: F, t126728: F, t126731: F, t24980: F, t2862: F, t5299: F, t6318: F, t856: F) -> (F, F) {
    let t126732 = -t126697 - t126701 - t126705 + 15.0 / 16.0 * t126709 + 4.0 * t126712 + 8.0 / 3.0 * t126715 - 8.0 / 9.0 * t126718 + 3.0 / 2.0 * t126723 + t126726 + t126728 - t126731;
    let t126736 = t24980 * t2862 * t6318 * t5299 * t856;
    (t126732, t126736)
}
