//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1210/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1210<F: Float>(t101898: F, t11520: F, t5674: F, t5675: F, t8411: F, t101858: F, t101864: F, t101869: F, t101873: F, t101876: F, t101879: F, t101883: F, t101886: F, t101891: F, t101896: F, t100210: F, t100264: F, t100290: F, t100327: F, t100369: F, t100415: F, t100464: F, t101582: F, t101625: F, t101671: F, t101698: F, t101739: F, t101769: F, t101809: F, t101854: F) -> (F, F) {
    let t101899 = t101898 / 27.0;
    let t101902 = t5674 * t8411 * t5675 * t11520;
    let t101904 = -t101858 / 36.0 - t101864 / 9.0 + t101869 / 24.0 + 2.0 / 9.0 * t101873 + 8.0 / 27.0 * t101876 + 4.0 / 27.0 * t101879 - t101883 - t101886 / 4.0 - t101891 / 3.0 - t101896 / 4.0 - t101899 - 2.0 * t101902;
    let t101908 = t100210 + t100264 + t100290 + t100327 + t100369 + t100415 + t100464 + t101582 + t101625 + t101671 + t101698 + t101739 + t101769 + t101809 + t101854 + t101904;
    (t101902, t101908)
}
