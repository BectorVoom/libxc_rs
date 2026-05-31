//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1401/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1401<F: Float>(t1246: F, t13068: F, t247: F, t3372: F, t3634: F, t1261: F, t3368: F, t3636: F, t3647: F, t3367: F, t414: F, t11239: F, t1243: F) -> (F, F, F, F, F, F) {
    let t13069 = t13068 * t1246;
    let t13085 = t247 * t3634 * t3372;
    let t13086 = t1261 * t13085;
    let t13089 = t247 * t3634 * t3368;
    let t13090 = t1261 * t13089;
    let t13092 = t3647 * t3636;
    let t13099 = F::cast_from(1.0_f64) / t414 / t3367;
    let t13126 = t11239 * t1243;
    (t13069, t13086, t13090, t13092, t13099, t13126)
}
