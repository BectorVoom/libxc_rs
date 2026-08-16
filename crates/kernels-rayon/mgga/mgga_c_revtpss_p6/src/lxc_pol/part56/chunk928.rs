//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 928/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk928(t32128: f64, t8599: f64, t2014: f64, t10301: f64, t8435: f64, t644: f64, t8441: f64, t8621: f64, t36: f64, t606: f64, t640: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32129 = t8599 * t32128;
    let t32131 = 2.0_f64 * t2014 * t32129;
    let t32132 = t10301 * t8435;
    let t32137 = t8441 * t644;
    let t32138 = t8621 * t32137;
    let t32143 = t8441 * t36;
    let t32145 = t8621 * t32143 * t606;
    let t32156 = t8621 * t84 * t640;
    (t32129, t32131, t32132, t32138, t32143, t32145, t32156)
}
