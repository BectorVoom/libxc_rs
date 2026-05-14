//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1029/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1029<F: Float>(t14689: F, t14708: F, t4227: F, t810: F, t2376: F, t2409: F, t13955: F, t13965: F, t14674: F, t14678: F, t14680: F, t14685: F, t14693: F, t14699: F, t14706: F, t14714: F, t2408: F) -> (F, F, F) {
    let t14974 = 7.0 / 144.0 * t14689;
    let t14978 = 7.0 / 144.0 * t14708;
    let t14979 = t4227 * t810;
    let t14981 = t2409 * t2376 * t14979;
    let t14985 = t14674 / 48.0 + t14678 / 48.0 + t14680 / 48.0 + t14685 / 768.0 - t14974 - t14693 / 1536.0 + t13955 + t14699 / 384.0 + t13965 + t14706 / 384.0 - t14978 + t2408 * t14981 / 48.0 - t14714 / 24.0;
    (t14979, t14981, t14985)
}
