//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2093/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093<F: Float>(t1497: F, t6977: F, t1926: F, t1927: F, t4241: F, t25163: F, t7715: F, t644: F, t7719: F, t13272: F, t607: F, t10301: F, t1470: F) -> (F, F, F, F, F, F) {
    let t101214 = t6977 * t1497;
    let t101215 = t1926 * t101214;
    let t101218 = t1927 * t4241;
    let t101219 = t1926 * t101218;
    let t101222 = t7715 * t25163;
    let t101226 = t7719 * t644;
    let t101227 = t1926 * t101226;
    let t101230 = t13272 * t607;
    let t101237 = t10301 * t1470;
    (t101215, t101219, t101222, t101227, t101230, t101237)
}
