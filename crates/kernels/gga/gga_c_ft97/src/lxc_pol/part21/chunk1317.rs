//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1317/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1317<F: Float>(t106121: F, t120009: F, t120013: F, t120017: F, t120021: F, t120025: F, t120029: F, t120033: F, t120037: F, t120040: F, t120044: F, t120048: F, t105856: F, t106127: F, t106128: F, t106133: F, t120055: F, t120059: F, t120061: F, t120066: F, t120070: F, t120074: F, t120080: F, t95301: F) -> (F, F) {
    let t121040 = -t106121 - t120009 / 54.0 - t120013 / 3.0 + t120017 / 18.0 - 2.0 * t120021 + 2.0 / 3.0 * t120025 + 2.0 / 3.0 * t120029 + 4.0 / 3.0 * t120033 + 4.0 / 3.0 * t120037 - 4.0 / 9.0 * t120040 - 2.0 * t120044 + t120048 / 9.0;
    let t121051 = t120055 / 2.0 - 2.0 * t120059 - 2.0 / 81.0 * t120061 + t106127 + t106128 - t120066 / 6.0 - t120070 / 4.0 + 4.0 / 81.0 * t95301 + 2.0 / 3.0 * t120074 - 4.0 / 27.0 * t105856 + t106133 + t120080 / 6.0;
    (t121040, t121051)
}
