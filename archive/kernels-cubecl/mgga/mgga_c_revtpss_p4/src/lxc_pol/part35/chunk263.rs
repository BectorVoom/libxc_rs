//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 263/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk263<F: Float>(t418: F, t408: F, t409: F, t1118: F, t406: F, t281: F, t414: F, t926: F, t240: F, t462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1129 = t418 * t418;
    let t1130 = F::cast_from(1.0_f64) / t1129;
    let t1131 = t408 * t1130;
    let t1132 = F::cast_from(1.0_f64) / t409;
    let t1137 = F::cast_from(0.29896666666666666667e0_f64) * t1118;
    let t1139 = F::sqrt(t406);
    let t1143 = t281 * t926 * t414;
    let t1144 = F::cast_from(0.82156666666666666667e-1_f64) * t1143;
    let t1145 = t240 * t462;
    (t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145)
}
