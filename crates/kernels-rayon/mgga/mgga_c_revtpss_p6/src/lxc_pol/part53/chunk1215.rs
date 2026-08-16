//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1215/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1215(t18227: f64, t8749: f64, t32866: f64, t4248: f64, t32822: f64, t7935: f64, t28021: f64, t8764: f64, t122820: f64, t27154: f64, t125402: f64, t125405: f64, t125407: f64, t125409: f64, t125410: f64, t125415: f64, t125417: f64, t1453: f64, t34462: f64) -> f64 {
    let t129277 = t18227 * t8749;
    let t129279 = t4248 * t32866;
    let t129281 = t32822 * t7935;
    let t129283 = t8764 * t28021;
    let t129285 = t122820 * t27154;
    let t129288 = t1453 * t34462 + 3.0_f64 * t125402 - t125405 - t125407 - t125409 + 3.0_f64 * t125410 + t125415 - t125417 - 2.0_f64 * t129277 - 2.0_f64 * t129279 + t129281 + t129283 - 3.0_f64 * t129285;
    t129288
}
