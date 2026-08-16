//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 884/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk884<F: Float>(t1699: F, t1102: F, t198: F, t3336: F, t336: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6213: F, t6215: F, t6217: F, t6221: F, t6225: F, t6229: F, t6396: F) -> (F, F) {
    let t6400 = t1699 * t1699;
    let t6404 = t1102 * t198 * t336 * t6396 - t198 * t3336 * t336 * t6400 - t6106 + t6108 - t6112 + t6144 + t6147 + t6213 + t6215 - t6217 + t6221 - t6225 - t6229;
    (t6400, t6404)
}
