//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 964/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk964<F: Float>(t2150: F, t29109: F, t473: F, t2142: F, t5245: F, t7637: F, t1243: F, t8190: F, t1248: F, t1287: F, t1811: F, t3140: F) -> (F, F, F, F) {
    let t29111 = t2150 * t473 * t29109;
    let t29118 = t2142 * t5245;
    let t29119 = t7637 * t29118;
    let t29122 = t1243 * t8190;
    let t29124 = t29122 * t1248 * t1287;
    let t29127 = t1811 * t3140;
    (t29111, t29119, t29124, t29127)
}
