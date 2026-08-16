//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1960/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1960<F: Float>(t29118: F, t7637: F, t1243: F, t8190: F, t1248: F, t1287: F, t1811: F, t3140: F, t1276: F, t2148: F, t5412: F, t1032: F) -> (F, F, F, F, F, F) {
    let t29119 = t7637 * t29118;
    let t29122 = t1243 * t8190;
    let t29124 = t29122 * t1248 * t1287;
    let t29127 = t1811 * t3140;
    let t29129 = t2148 * t29127 * t1276;
    let t29132 = t2148 * t5412;
    let t29135 = t1811 * t1032;
    (t29119, t29122, t29124, t29129, t29132, t29135)
}
