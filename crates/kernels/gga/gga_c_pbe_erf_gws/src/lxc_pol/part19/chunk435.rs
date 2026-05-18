//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 435/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk435<F: Float>(t202: F, t631: F, t184: F, t582: F, t611: F, t185: F, t1687: F, t174: F, t177: F, t332: F, t395: F, t574: F) -> (F, F, F, F, F, F, F, F) {
    let t1729 = t202 * t631;
    let t1730 = t1729 * t184;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1743 = F::new(0.25188888888888888889e-2) * t1687;
    let t1754 = t174 * t332 * t177;
    let t1755 = F::new(0.25188888888888888889e-2) * t1754;
    let t1756 = t395 * t574;
    (t1729, t1730, t1740, t1741, t1743, t1754, t1755, t1756)
}
