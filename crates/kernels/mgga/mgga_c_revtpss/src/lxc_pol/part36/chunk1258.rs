//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1258/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1258<F: Float>(t7058: F, t99201: F, t2435: F, t7774: F, t25431: F, t2439: F, t7759: F, t780: F, t785: F, t25411: F, t1711: F, t2411: F) -> (F, F, F, F, F) {
    let t99481 = t7058 * t99201;
    let t99495 = t7774 * t2435;
    let t99496 = t25431 * t99495;
    let t99520 = t2439 * t785 * t7759 * t780;
    let t99522 = t25411 * t99495;
    let t100987 = t2411 * t1711;
    (t99481, t99496, t99520, t99522, t100987)
}
