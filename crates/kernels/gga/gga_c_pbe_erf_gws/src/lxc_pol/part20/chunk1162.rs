//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1162/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1162<F: Float>(t11501: F, t14567: F, t6608: F, t55486: F, t56954: F, t56956: F, t56958: F, t56960: F, t56962: F, t56964: F, t56966: F, t56968: F, t56970: F, t56972: F, t11615: F, t14011: F) -> (F, F) {
    let t56975 = t6608 * t11501 * t14567;
    let t56977 = t56954 / 24.0 - t56956 / 48.0 + t56958 / 128.0 - t56960 / 48.0 + t56962 / 96.0 - t56964 / 384.0 + 5.0 / 96.0 * t56966 + t56968 / 24.0 + t56970 / 384.0 - t56972 / 384.0 + t56975 / 96.0 + t55486;
    let t56978 = t14011 * t11615;
    (t56977, t56978)
}
