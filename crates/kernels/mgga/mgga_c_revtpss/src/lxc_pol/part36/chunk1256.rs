//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1256/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1256<F: Float>(t136: F, t2457: F, t7778: F, t25299: F, t1568: F, t786: F, t25410: F, t25375: F, t99365: F, t10073: F, t1579: F, t1958: F, t25390: F) -> (F, F, F, F, F, F) {
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99412 = t25375 * t99365;
    let t99423 = t10073 * t25390 * t1958 * t1579;
    (t99380, t99381, t99403, t99404, t99412, t99423)
}
