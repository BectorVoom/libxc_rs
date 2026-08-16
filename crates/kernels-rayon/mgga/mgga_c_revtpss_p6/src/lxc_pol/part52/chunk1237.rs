//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1237/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1237(t32392: f64, t7984: f64, t32394: f64, t28704: f64, t8634: f64, t127381: f64, t25082: f64, t26405: f64, t28173: f64, t8698: f64, t102019: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128317 = 2.0_f64 * t32392 * t7984;
    let t128319 = 2.0_f64 * t32394 * t7984;
    let t128321 = 2.0_f64 * t8634 * t28704;
    let t128324 = 3.0_f64 * t25082 * t26405 * t127381;
    let t128326 = 3.0_f64 * t8698 * t28173;
    let t128331 = t102019 * t1936;
    (t128317, t128319, t128321, t128324, t128326, t128331)
}
