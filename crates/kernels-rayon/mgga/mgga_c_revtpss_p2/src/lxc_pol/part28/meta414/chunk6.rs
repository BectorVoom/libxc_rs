//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1574/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1574(t15513: f64, t291: f64, t11399: f64, t15406: f64, t15413: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15495: f64, t1622: f64, t2938: f64, t2963: f64, t2971: f64, t2989: f64, t4647: f64, t4670: f64) -> (f64, f64) {
    let t15515 = 0.621814e-1_f64 * t15513 * t291;
    let t15516 = 1.0_f64 * t4647 * t2963 + 0.32163958997385070134e2_f64 * t15406 * t2971 + 1.0_f64 * t11399 * t1622 + 2.0_f64 * t2938 * t4670 - 0.11696447245269292414e1_f64 * t15413 * t2989 - t15418 - t15420 - t15423 - t15425 - t15427 - t15477 - 0.19751673498613801407e-1_f64 * t15495 + t15515;
    (t15515, t15516)
}
