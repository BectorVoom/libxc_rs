//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2253/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2253<F: Float>(t644: F, t7719: F, t1926: F, t13272: F, t607: F, t2248: F, t77: F, t7705: F, t10301: F, t1470: F, t2247: F, t4181: F) -> (F, F, F, F, F) {
    let t101226 = t7719 * t644;
    let t101227 = t1926 * t101226;
    let t101230 = t13272 * t607;
    let t101234 = t77 * t7705 * t2248;
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    (t101227, t101230, t101234, t101237, t101240)
}
