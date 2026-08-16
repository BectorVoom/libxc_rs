//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3933/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3933(t13426: f64, t13514: f64, t13517: f64, t13537: f64, t1519: f64, t18163: f64, t18227: f64, t1843: f64, t1911: f64, t21882: f64, t21891: f64, t3813: f64, t3821: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t49686: f64, t508: f64, t5887: f64, t5920: f64, t651: f64, t6934: f64, t75485: f64, t75494: f64, t75657: f64, t75667: f64, t94: f64) -> f64 {
    let t75714 = -4.0_f64 * t13514 * t1843 * t651 - 2.0_f64 * t3813 * t5920 * t651 - 2.0_f64 * t508 * t651 * t75657 - 4.0_f64 * t508 * t75494 * t94 - 8.0_f64 * t13426 * t4257 - 8.0_f64 * t13426 * t4293 + 2.0_f64 * t13517 * t1911 - 4.0_f64 * t13537 * t4248 - 4.0_f64 * t1519 * t49686 - 4.0_f64 * t1519 * t75485 - 8.0_f64 * t1519 * t75667 - 4.0_f64 * t18163 * t5887 - 8.0_f64 * t18227 * t4257 - 8.0_f64 * t18227 * t4293 - 4.0_f64 * t21882 * t4254 - 8.0_f64 * t21891 * t4254 + t3821 * t6934;
    t75714
}
