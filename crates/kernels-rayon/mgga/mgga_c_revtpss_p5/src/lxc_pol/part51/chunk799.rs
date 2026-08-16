//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 799/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk799(t3215: f64, t7117: f64, t3123: f64, t7121: f64, t365: f64, t3089: f64, t1087: f64, t1024: f64, t7131: f64, t3167: f64, t7120: f64, t1033: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25498 = t7117 * t3215;
    let t25512 = t3123 * t7121;
    let t25515 = sigma0 * t365;
    let t25516 = t25515 * t3089;
    let t25517 = t1087 * t25516;
    let t25522 = t1024 * t7131;
    let t25525 = t7120 * t3167;
    let t25526 = t1033 * t25525;
    (t25498, t25512, t25515, t25516, t25517, t25522, t25526)
}
