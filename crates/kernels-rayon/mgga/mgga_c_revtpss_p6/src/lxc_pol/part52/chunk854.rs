//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 854/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk854(t3123: f64, t7121: f64, t365: f64, t3089: f64, t1087: f64, t1024: f64, t7131: f64, t3167: f64, t7120: f64, t1033: f64, t3173: f64, t7122: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25512 = t3123 * t7121;
    let t25515 = sigma0 * t365;
    let t25516 = t25515 * t3089;
    let t25517 = t1087 * t25516;
    let t25522 = t1024 * t7131;
    let t25525 = t7120 * t3167;
    let t25526 = t1033 * t25525;
    let t25529 = t7122 * t3173;
    (t25512, t25515, t25516, t25517, t25522, t25526, t25529)
}
