//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 541/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk541<F: Float>(t1137: F, t1403: F, t1427: F, t1454: F, t247: F, t6001: F, t6002: F, t6745: F, t6749: F, t6754: F, t6840: F, t6844: F, t6913: F, t6926: F, t6931: F, t6941: F, t6945: F, t6947: F) -> (F,) {
    let t6953 = t6745 * t1427 / 6.0 - t6001 - t6002 * t6749 / 18.0 - t1403 * t6754 / 3.0 + t1403 * t6840 / 6.0 + t1403 * t6844 / 6.0 - t1137 * t1454 - t247 * t6945 + 2.0 * t6947 - 2.0 * t6913 - 2.0 * t6926 + 4.0 * t6931 - 2.0 * t6941;
    (t6953,)
}
