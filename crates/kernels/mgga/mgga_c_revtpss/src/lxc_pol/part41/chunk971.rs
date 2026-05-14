//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 971/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk971<F: Float>(t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F, t557: F, t1429: F, t9292: F, t4096: F, t9285: F, t1398: F, t215: F, t268: F, t543: F) -> (F, F, F, F, F, F, F, F) {
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    let t10114 = 0.19637199382202157274e-3 * t10111 * t1428 * t22;
    let t10115 = t22 * t2452;
    let t10117 = 0.11044544084478153697e-3 * t10115 * t557;
    let t10126 = 0.17073386770573548589e-1 * t9292 * t1429;
    let t10129 = 0.46263278077393568556e-2 * t3964 * t4096 * t9285;
    let t10136 = t268 * t215 * t1398 * t543;
    (t10109, t10111, t10114, t10115, t10117, t10126, t10129, t10136)
}
