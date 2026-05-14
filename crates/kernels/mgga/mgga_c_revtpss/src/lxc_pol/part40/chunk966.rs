//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 966/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk966<F: Float>(t1432: F, t1433: F, t9288: F, t4066: F, t72: F, t686: F, t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F, t557: F) -> (F, F, F, F, F, F, F) {
    let t10102 = 0.30356481678079769392e-1 * t1432 * t1433 * t9288;
    let t10103 = t4066 * t72;
    let t10105 = t1432 * t10103 * t686;
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    let t10114 = 0.19637199382202157274e-3 * t10111 * t1428 * t22;
    let t10115 = t22 * t2452;
    let t10117 = 0.11044544084478153697e-3 * t10115 * t557;
    (t10102, t10105, t10109, t10111, t10114, t10115, t10117)
}
