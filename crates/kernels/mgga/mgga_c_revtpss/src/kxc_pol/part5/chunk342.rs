//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 342/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk342<F: Float>(t1119: F, t1124: F, t422: F, t418: F) -> (F, F, F, F) {
    let t1126 = -t1119 + 0.17808333333333333333e-1 * t1124;
    let t1128 = 0.621814e-1 * t1126 * t422;
    let t1129 = t418 * t418;
    let t1130 = 1.0 / t1129;
    (t1126, t1128, t1129, t1130)
}
