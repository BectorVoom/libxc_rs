//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 856/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk856(t4066: f64, t72: f64, t1432: f64, t686: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64, t1428: f64, t22: f64) -> (f64, f64, f64, f64) {
    let t10103 = t4066 * t72;
    let t10105 = t1432 * t10103 * t686;
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    let t10114 = 0.19637199382202157274e-3_f64 * t10111 * t1428 * t22;
    (t10105, t10109, t10111, t10114)
}
