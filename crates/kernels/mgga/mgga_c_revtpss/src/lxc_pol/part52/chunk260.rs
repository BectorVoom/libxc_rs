//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 260/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk260<F: Float>(t1119: F, t1124: F, t422: F, t418: F, t408: F, t409: F, t1118: F, t406: F, t281: F, t414: F, t926: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1126 = -t1119 + F::cast_from(0.17808333333333333333e-1_f64) * t1124;
    let t1128 = F::new(0.621814e-1) * t1126 * t422;
    let t1129 = t418 * t418;
    let t1130 = F::new(1.0) / t1129;
    let t1131 = t408 * t1130;
    let t1132 = F::new(1.0) / t409;
    let t1134 = -t1118 / F::new(3.0) + t1124 / F::new(3.0);
    let t1135 = t1132 * t1134;
    let t1137 = F::cast_from(0.29896666666666666667e0_f64) * t1118;
    let t1139 = F::sqrt(t406);
    let t1140 = t1139 * t1134;
    let t1143 = t281 * t926 * t414;
    (t1126, t1128, t1129, t1130, t1131, t1132, t1134, t1135, t1137, t1139, t1140, t1143)
}
