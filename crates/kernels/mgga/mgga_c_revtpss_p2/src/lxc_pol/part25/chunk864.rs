//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 864/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk864<F: Float>(t10190: F, t9397: F, t9557: F, t9589: F, t2327: F, t648: F, t64: F, t843: F, t112: F, t2289: F, t666: F, t2341: F, t625: F) -> (F, F, F, F, F, F) {
    let t10192 = t9397 + t9557 + t9589 + t10190;
    let t10194 = t648 * t2327;
    let t10199 = t64 * t843;
    let t10201 = F::new(154.0) / F::new(27.0) * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10204 = t625 * t2341;
    (t10192, t10194, t10199, t10201, t10202, t10204)
}
