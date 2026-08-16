//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 875/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk875(t10073: f64, t4496: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64) -> (f64, f64, f64, f64, f64) {
    let t14512 = t10073 * t4496;
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    let t14557 = t2777 * t4518;
    let t14558 = t2439 * t14557;
    let t14563 = t4499 * t2470;
    (t14512, t14525, t14533, t14558, t14563)
}
