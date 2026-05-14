//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 479/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk479<F: Float>(t1514: F, t625: F, t1513: F, t2339: F, t1504: F, t2349: F, t1509: F, t2357: F, t1534: F, t72: F, t757: F, t1469: F, t750: F, t706: F, t1531: F, t705: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4261 = t625 * t1514;
    let t4263 = t2339 * t1513;
    let t4269 = t2349 * t1504;
    let t4279 = t2357 * t1509;
    let t4302 = t1534 * t72;
    let t4303 = t4302 * t757;
    let t4305 = t750 * t1469;
    let t4306 = t706 * t4305;
    let t4311 = t705 * t1531;
    (t4261, t4263, t4269, t4279, t4302, t4303, t4305, t4306, t4311)
}
