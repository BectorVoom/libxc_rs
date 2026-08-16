//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 734/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk734(t786: f64, t9679: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64, t2237: f64, t240: f64, t550: f64, t816: f64, t1379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9680 = t786 * t9679;
    let t9691 = 0.17073386770573548589e-1_f64 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = 0.30356481678079769392e-1_f64 * t1362 * t9692;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    let t9711 = 0.12846167376791569079e-2_f64 * t1379 * t9709;
    (t9680, t9691, t9692, t9694, t9707, t9709, t9711)
}
