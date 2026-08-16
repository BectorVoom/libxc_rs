//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1566/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1566(t1234: f64, t24680: f64, t1222: f64, t140: f64, t24826: f64, t1209: f64, t24864: f64, t473: f64, t24704: f64, t3153: f64, t13045: f64, t6622: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84185 = t1234 * t24680;
    let t84195 = t1222 * t140 * t24826;
    let t84315 = t1209 * t24864;
    let t84429 = t473 * t24864;
    let t84487 = t24704 * t3153;
    let t84636 = t13045 * t6622;
    (t84185, t84195, t84315, t84429, t84487, t84636)
}
