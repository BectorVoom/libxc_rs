//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 885/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk885(t18814: f64, t689: f64, t6042: f64, t786: f64, t789: f64, t6049: f64, t779: f64, t14987: f64, t4481: f64, t6075: f64, t892: f64, t262: f64, t5962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18815 = t689 * t18814;
    let t18821 = t786 * t6042;
    let t18822 = t18821 * t789;
    let t18825 = t779 * t6049;
    let t18826 = t689 * t18825;
    let t18828 = t14987 * t4481;
    let t18850 = t6075 * t892;
    let t18860 = t262 * t5962;
    (t18815, t18822, t18826, t18828, t18850, t18860)
}
