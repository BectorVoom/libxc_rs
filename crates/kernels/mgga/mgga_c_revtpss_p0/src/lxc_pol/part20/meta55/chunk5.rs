//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 381/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk381<F: Float>(t1126: F, t422: F, t418: F, t408: F, t409: F, t1118: F, t1124: F) -> (F, F, F, F, F, F) {
    let t1128 = F::new(0.621814e-1) * t1126 * t422;
    let t1129 = t418 * t418;
    let t1130 = F::new(1.0) / t1129;
    let t1131 = t408 * t1130;
    let t1132 = F::new(1.0) / t409;
    let t1134 = -t1118 / F::new(3.0) + t1124 / F::new(3.0);
    (t1128, t1129, t1130, t1131, t1132, t1134)
}
