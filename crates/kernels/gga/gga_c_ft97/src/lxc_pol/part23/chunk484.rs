//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 484/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk484<F: Float>(t1403: F, t1427: F, t1454: F, t247: F, t5996: F, t6001: F, t6002: F, t6005: F, t6011: F, t6064: F, t6068: F, t6155: F, t6171: F, t6176: F, t6188: F, t6192: F, t6194: F, t719: F) -> (F,) {
    let t6200 = t5996 * t1427 / 6.0 - t6001 - t6002 * t6005 / 18.0 - t1403 * t6011 / 3.0 + t1403 * t6064 / 6.0 + t1403 * t6068 / 6.0 - t719 * t1454 - t247 * t6192 + 2.0 * t6194 - 2.0 * t6155 - 2.0 * t6171 + 4.0 * t6176 - 2.0 * t6188;
    (t6200,)
}
