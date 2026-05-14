//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 967/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk967<F: Float>(t10024: F, t268: F, t543: F, t4101: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t1385: F, t4066: F, t1398: F, t215: F, t2453: F, t4100: F, t281: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t10119 = t268 * t10024 * t543;
    let t10120 = t4101 * t10119;
    let t10126 = 0.17073386770573548589e-1 * t9292 * t1429;
    let t10129 = 0.46263278077393568556e-2 * t3964 * t4096 * t9285;
    let t10130 = t1385 * t4066;
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    (t10120, t10126, t10129, t10130, t10137, t10139, t10142)
}
