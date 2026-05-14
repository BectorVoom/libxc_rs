//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1354/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1354<F: Float>(t113447: F, t113466: F, t126983: F, t126986: F, t126991: F, t126995: F, t126999: F, t127002: F, t99525: F, t99535: F, t99795: F, t99802: F, t31608: F, t681: F, t89: F, t31624: F, t375: F) -> (F, F, F, F) {
    let t127004 = -t126983 - t99795 + t99525 + t113447 + t113466 + t126986 / 3.0 + t126991 / 3.0 + t126995 / 3.0 + t99535 - t99802 - t126999 + 4.0 * t127002;
    let t127007 = t89 * t681 * t31608;
    let t127008 = 2.0 * t127007;
    let t127010 = t89 * t375 * t31624;
    (t127004, t127007, t127008, t127010)
}
