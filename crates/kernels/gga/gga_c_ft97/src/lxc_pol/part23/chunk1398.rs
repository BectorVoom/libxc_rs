//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1398/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1398<F: Float>(t114292: F, t126781: F, t126783: F, t126787: F, t126789: F, t126793: F, t126797: F, t126801: F, t126804: F, t126807: F, t126810: F, t99315: F, t114312: F, t114314: F, t114318: F, t114320: F, t114328: F, t114337: F, t114340: F, t126814: F, t126817: F, t126821: F, t126824: F, t99317: F) -> (F, F) {
    let t128186 = 2.0 / 27.0 * t126781 + 2.0 / 27.0 * t126783 - 4.0 * t126787 + 2.0 / 27.0 * t126789 - 4.0 / 9.0 * t126793 + t114292 + 2.0 / 81.0 * t99315 - 2.0 / 9.0 * t126797 - 4.0 / 27.0 * t126801 - 4.0 / 9.0 * t126804 - 4.0 / 9.0 * t126807 + 8.0 / 9.0 * t126810;
    let t128192 = -2.0 / 9.0 * t126814 - t126817 / 9.0 - t114312 - t114314 - t126821 / 18.0 + t114318 + t114320 + t114328 + 4.0 / 81.0 * t99317 - 2.0 / 9.0 * t126824 + t114337 + t114340;
    (t128186, t128192)
}
