//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2222/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2222(t1497: f64, t4237: f64, t77: f64, t1493: f64, t4241: f64, t5872: f64, t640: f64, t21809: f64, t84: f64, t1925: f64, t2247: f64, t5819: f64) -> (f64, f64, f64, f64, f64) {
    let t108733 = t77 * t4237 * t1497;
    let t108737 = t77 * t1493 * t4241;
    let t108745 = t77 * t640 * t5872;
    let t108749 = t77 * t84 * t21809;
    let t108753 = t2247 * t5819 * t1925;
    (t108733, t108737, t108745, t108749, t108753)
}
