//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2810/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2810<F: Float>(t11043: F, t2453: F, t10506: F, t2458: F, t2761: F, t11029: F, t9303: F, t39501: F, t781: F, t10510: F, t11044: F, t252: F, t257: F, t268: F, t39644: F, t8779: F) -> (F, F, F, F, F, F, F) {
    let t41020 = t2453 * t11043;
    let t41021 = t41020 * t10506;
    let t41029 = t2453 * t2761 * t2458;
    let t41034 = t9303 * t11029;
    let t41037 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t781;
    let t41038 = t11044 * t10510;
    let t41049 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t252 * t257 * t8779 * t268;
    (t41020, t41021, t41029, t41034, t41037, t41038, t41049)
}
