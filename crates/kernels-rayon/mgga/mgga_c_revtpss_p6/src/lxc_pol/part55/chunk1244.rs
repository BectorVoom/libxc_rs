//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1244/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1244(t2089: f64, t28042: f64, t651: f64, t2322: f64, t34028: f64, t4254: f64, t1518: f64, t32575: f64, t28043: f64, t7359: f64, t34243: f64, t7235: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128552 = 2.0_f64 * t651 * t2089 * t28042;
    let t128554 = 2.0_f64 * t2322 * t34028;
    let t128557 = 2.0_f64 * t4254 * t34028;
    let t128560 = 2.0_f64 * t651 * t32575 * t1518;
    let t128562 = 2.0_f64 * t7359 * t28043;
    let t128572 = t7235 * t34243;
    (t128552, t128554, t128557, t128560, t128562, t128572)
}
