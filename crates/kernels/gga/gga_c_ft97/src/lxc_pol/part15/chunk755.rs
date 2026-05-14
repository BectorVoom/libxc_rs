//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 755/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk755<F: Float>(t10594: F, t15011: F, t15025: F, t22302: F, t22306: F, t22310: F, t22313: F, t22316: F, t22319: F, t22321: F, t22323: F, t22326: F, t462: F, t92: F, t22301: F, t845: F, t91: F) -> (F, F) {
    let t22329 = -2.0 * t462 * t22302 - t10594 - t92 * t22306 - 4.0 / 9.0 * t15025 - 4.0 / 3.0 * t15011 + 2.0 / 3.0 * t462 * t22310 + 4.0 / 3.0 * t462 * t22313 - 2.0 / 3.0 * t462 * t22316 + t462 * t22319 + t462 * t22321 - 2.0 * t462 * t22323 + 2.0 * t462 * t22326;
    let t22330 = t22301 + t22329;
    let t22332 = t91 * t845 * t22330;
    (t22330, t22332)
}
