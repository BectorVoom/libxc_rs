//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 853/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk853(t1811: f64, t5219: f64, t1284: f64, t6564: f64, t473: f64, t6695: f64, t20849: f64, t487: f64, t5812: f64, t602: f64, t1469: f64, t70: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21394 = t5219 * t1811;
    let t21439 = t6564 * t1284;
    let t21541 = t473 * t6695;
    let t21621 = t20849 * t487;
    let t21663 = t5812 * t602;
    let t21686 = t1469 * t70 * t72;
    (t21394, t21439, t21541, t21621, t21663, t21686)
}
