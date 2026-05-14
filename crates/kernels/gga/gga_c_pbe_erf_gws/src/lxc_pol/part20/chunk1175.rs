//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1175/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1175<F: Float>(t3867: F, t4033: F, t51388: F, t51396: F, t54302: F, t55582: F, t57127: F, t57130: F, t57132: F, t57134: F, t57138: F, t57140: F, t57142: F, t57144: F, t11573: F, t14015: F) -> (F, F) {
    let t57146 = t4033 * t3867;
    let t57148 = -t57127 / 4.0 + t57130 / 8.0 + t57132 / 48.0 - t57134 / 384.0 - 119.0 / 3456.0 * t51388 - 119.0 / 1728.0 * t51396 + t57138 / 24.0 + t54302 - t55582 - t57140 / 768.0 - 7.0 / 144.0 * t57142 - 7.0 / 48.0 * t57144 + 7.0 / 144.0 * t57146;
    let t57151 = t14015 * t11573;
    (t57148, t57151)
}
