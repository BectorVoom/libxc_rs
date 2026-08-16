//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1005/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1005(t14485: f64, t2465: f64, t10073: f64, t4496: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t10867: f64, t225: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14486 = t2465 * t14485;
    let t14512 = t10073 * t4496;
    let t14523 = t1559 * t136;
    let t14524 = t14523 * t2457;
    let t14525 = t10535 * t14524;
    let t14533 = t10069 * t4496;
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    (t14486, t14512, t14523, t14524, t14525, t14533, t14545, t14546)
}
