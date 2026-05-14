//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 635/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk635<F: Float>(t14683: F, t10400: F, t10279: F, t1186: F, t89: F, t9733: F, t13730: F, t4044: F, t1471: F, t4092: F) -> (F, F, F, F, F, F) {
    let t14684 = 2.0 / 9.0 * t14683;
    let t14708 = 4.0 / 27.0 * t10400;
    let t14711 = 4.0 / 81.0 * t10279;
    let t14715 = t89 * t9733 * t1186;
    let t14718 = t89 * t13730 * t4044;
    let t14721 = t4092 * t1471;
    (t14684, t14708, t14711, t14715, t14718, t14721)
}
