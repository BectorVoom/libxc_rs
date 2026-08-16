//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 925/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk925(t20850: f64, t480: f64, t3520: f64, t6534: f64, t5265: f64, t5274: f64, t12916: f64, t6689: f64, t3718: f64, t1219: f64, t6667: f64, t247: f64, t3634: f64, t6429: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20851 = t20850 * t480;
    let t20895 = t3520 * t6534;
    let t20917 = t5274 * t5265;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    let t20966 = t6667 * t1219;
    let t20973 = t247 * t3634 * t6429;
    (t20851, t20895, t20917, t20926, t20927, t20966, t20973)
}
