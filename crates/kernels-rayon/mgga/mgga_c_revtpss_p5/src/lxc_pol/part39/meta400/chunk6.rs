//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1467/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1467(t16630: f64, t18152: f64, t2371: f64, t94: f64, t118: f64, t1310: f64, t1315: f64, t13425: f64, t13426: f64, t13429: f64, t14310: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t2320: f64, t2322: f64, t2331: f64, t3821: f64, t4151: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t508: f64, t511: f64, t5517: f64, t5787: f64, t649: f64, t671: f64) -> (f64, f64, f64) {
    let t18153 = t16630 + t18152;
    let t18163 = t94 * t2371;
    let t18176 = -t118 * t18153 - 2.0_f64 * t1310 * t4246 + 2.0_f64 * t1315 * t5787 - t13425 * t508 - 4.0_f64 * t13426 * t671 - 2.0_f64 * t13429 * t508 + t14310 * t511 - 2.0_f64 * t1519 * t18163 - t1843 * t2320 + t1847 * t4151 + t1911 * t3821 - 4.0_f64 * t2322 * t4293 - 4.0_f64 * t2331 * t4248 - 4.0_f64 * t4254 * t4257 - 2.0_f64 * t5517 * t649;
    (t18153, t18163, t18176)
}
