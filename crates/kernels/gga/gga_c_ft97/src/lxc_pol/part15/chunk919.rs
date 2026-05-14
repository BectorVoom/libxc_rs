//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 919/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk919<F: Float>(t103: F, t108: F, t19972: F, t20162: F, t4414: F, t4501: F, t4621: F, t85406: F, t85797: F, t85882: F, t85895: F, t85903: F, t86010: F, t86321: F, t86404: F, t86411: F, t984: F) -> (F,) {
    let t86559 = -3.0 * t108 * t20162 * t4414 + 2.0 * t103 * t86404 - 8.0 * t19972 * t984 - 6.0 * t4501 * t4621 - 48.0 * t85406 + 48.0 * t85797 - 72.0 * t85882 + 16.0 * t85895 + 24.0 * t85903 - 8.0 * t86010 + 48.0 * t86321 - 12.0 * t86411;
    (t86559,)
}
