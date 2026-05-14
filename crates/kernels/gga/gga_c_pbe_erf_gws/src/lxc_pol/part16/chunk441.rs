//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 441/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk441<F: Float>(t1748: F, t203: F, t184: F, t221: F, t174: F, t177: F, t332: F, t395: F, t574: F, t56: F, t589: F) -> (F, F, F, F, F, F, F) {
    let t1749 = t203 * t1748;
    let t1750 = t1749 * t184;
    let t1752 = 2.0 / 15.0 * t1750 * t221;
    let t1754 = t174 * t332 * t177;
    let t1755 = 0.25188888888888888889e-2 * t1754;
    let t1756 = t395 * t574;
    let t1758 = t56 * t589;
    (t1749, t1750, t1752, t1754, t1755, t1756, t1758)
}
