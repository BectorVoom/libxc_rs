//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2073/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2073(t25431: f64, t99389: f64, t1568: f64, t786: f64, t25410: f64, t25413: f64, t25375: f64, t99365: f64, t1579: f64, t25392: f64, t4481: f64, t92921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99391 = 0.14456046980341999104e-1_f64 * t25431 * t99389;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99406 = 0.14456046980341999104e-1_f64 * t99404 * t25413;
    let t99412 = t25375 * t99365;
    let t99414 = t25392 * t1579;
    let t99420 = 0.19514881078765566038e-1_f64 * t92921 * t4481;
    (t99391, t99403, t99404, t99406, t99412, t99414, t99420)
}
