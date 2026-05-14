//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1205/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1205<F: Float>(t54126: F, t56954: F, t56956: F, t56958: F, t56960: F, t56962: F, t56964: F, t56966: F, t56968: F, t56970: F, t56972: F, t56975: F, t55487: F, t55491: F, t55500: F, t55508: F, t56978: F, t56980: F, t56982: F, t56984: F, t56986: F, t56988: F, t56990: F, t56992: F, t56994: F) -> (F, F) {
    let t58645 = t56954 / 12.0 - t56956 / 24.0 + t56958 / 64.0 - t56960 / 24.0 + t56962 / 48.0 - t56964 / 192.0 + 5.0 / 48.0 * t56966 + t56968 / 12.0 + t56970 / 192.0 - t56972 / 192.0 + t56975 / 48.0 + 119.0 / 864.0 * t54126;
    let t58655 = -t55487 + t56978 / 48.0 + t56980 / 12.0 - 7.0 / 36.0 * t56982 + 5.0 / 192.0 * t56984 - t55491 + t55500 + t56986 / 384.0 - 7.0 / 576.0 * t56988 + t55508 - 7.0 / 72.0 * t56990 + 7.0 / 36.0 * t56992 + t56994 / 48.0;
    (t58645, t58655)
}
