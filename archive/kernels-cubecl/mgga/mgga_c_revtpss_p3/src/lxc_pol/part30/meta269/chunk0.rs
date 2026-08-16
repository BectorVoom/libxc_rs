//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1181/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1181<F: Float>(t670: F, t7330: F, t572: F, t117: F, t7002: F, t2121: F, t38: F) -> (F, F, F, F, F) {
    let t7331 = t7330 * t670;
    let t7333 = F::cast_from(6.0_f64) * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = F::cast_from(3.0_f64) * t572 * t7334;
    let t7565 = t38 * t2121;
    (t7331, t7333, t7334, t7336, t7565)
}
