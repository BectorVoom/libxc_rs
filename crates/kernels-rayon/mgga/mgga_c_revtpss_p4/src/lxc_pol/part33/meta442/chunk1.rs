//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1612/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1612(t12254: f64, t20293: f64, t141: f64, t12542: f64, t12543: f64, t16710: f64, t16931: f64, t17131: f64, t17140: f64, t20366: f64, t20368: f64, t20371: f64, t20373: f64) -> (f64, f64) {
    let t20377 = t12254 * t20293;
    let t20378 = t141 * t20377;
    let t20380 = -0.412621875e-1_f64 * t20366 + 0.16504875e0_f64 * t20368 + 0.82524375e-1_f64 * t20371 - t17131 - t12542 - t12543 + 0.16504875e0_f64 * t20373 - 0.40256666666666666668e0_f64 * t16710 + t17140 + 0.36793333333333333333e-1_f64 * t16931 + 0.36793333333333333333e-1_f64 * t20378;
    (t20378, t20380)
}
