//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 356/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk356<F: Float>(t1134: F, t1139: F, t281: F, t414: F, t926: F, t240: F, t462: F) -> (F, F, F, F) {
    let t1140 = t1139 * t1134;
    let t1143 = t281 * t926 * t414;
    let t1144 = F::cast_from(0.82156666666666666667e-1_f64) * t1143;
    let t1145 = t240 * t462;
    (t1140, t1143, t1144, t1145)
}
