//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1575/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1575(t16892: f64, t16708: f64, t16710: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t16927: f64, t16931: f64, t16933: f64) -> (f64, f64) {
    let t17131 = 0.22076e0_f64 * t16892;
    let t17140 = 0.13418888888888888889e0_f64 * t16708;
    let t17148 = 0.36793333333333333333e-1_f64 * t16908 + 0.16504875e0_f64 * t16927 - 0.40256666666666666667e0_f64 * t16710 + t17140 + 0.36793333333333333334e-1_f64 * t16931 + 0.258925e1_f64 * t16933 - 0.12077e1_f64 * t16722 + 0.12077e1_f64 * t16740 + 0.60385e0_f64 * t16744 + 0.181155e1_f64 * t16735 + 0.33547222222222222222e0_f64 * t16717;
    (t17131, t17148)
}
