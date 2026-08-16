//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1054/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1054(t32540: f64, t32574: f64, t118: f64, t1448: f64, t2033: f64, t28286: f64, t28196: f64, t10301: f64, t8619: f64, t10309: f64, t644: f64, t8621: f64, t8622: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32575 = t32540 + t32574;
    let t32576 = t118 * t32575;
    let t32577 = t2033 * t1448;
    let t32578 = t28286 * t32577;
    let t32580 = 2.0_f64 * t28196 * t32578;
    let t32581 = t10301 * t8619;
    let t32584 = t10309 * t8619;
    let t32586 = t8621 * t8622 * t644;
    (t32575, t32576, t32577, t32578, t32580, t32581, t32584, t32586)
}
