//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1451/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1451(t10535: f64, t136: f64, t2457: f64, t5978: f64, t10069: f64, t18750: f64, t2783: f64, t6041: f64, t786: f64, t18689: f64, t2435: f64, t18688: f64, t2439: f64, t2777: f64) -> (f64, f64, f64, f64, f64) {
    let t62723 = t10535 * t5978 * t136 * t2457;
    let t62777 = t10069 * t18750;
    let t62808 = t786 * t2783 * t6041;
    let t62843 = t2435 * t18689;
    let t62847 = t2439 * t2777 * t18688;
    (t62723, t62777, t62808, t62843, t62847)
}
