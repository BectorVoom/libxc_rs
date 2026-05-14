//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 926/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk926<F: Float>(t1771: F, t2406: F, t41446: F, t41448: F, t92: F, t9568: F, t41454: F, t9570: F, t2404: F, t41464: F, t41693: F, t41696: F, t41700: F, t41703: F, t41705: F, t41707: F) -> (F, F, F, F, F, F, F, F) {
    let t41709 = t1771 * t2406;
    let t41711 = t41446 * t41448;
    let t41713 = t92 * t9568 * t41711;
    let t41716 = t92 * t9568 * t41454;
    let t41718 = t9570 * t41448;
    let t41720 = t92 * t2404 * t41718;
    let t41723 = t92 * t2404 * t41464;
    let t41725 = 8.0 * t41693 - 12.0 * t41696 + 2.0 * t41700 + 8.0 / 3.0 * t41703 + 112.0 / 81.0 * t41705 - 8.0 / 9.0 * t41707 - 16.0 / 27.0 * t41709 + 40.0 / 9.0 * t41713 - 20.0 / 9.0 * t41716 - 8.0 * t41720 + 8.0 * t41723;
    (t41709, t41711, t41713, t41716, t41718, t41720, t41723, t41725)
}
