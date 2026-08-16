//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1500/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1500(t118019: f64, t1312: f64, t13426: f64, t13435: f64, t1453: f64, t18163: f64, t18227: f64, t1911: f64, t2199: f64, t2201: f64, t2322: f64, t27123: f64, t31157: f64, t31158: f64, t31161: f64, t31169: f64, t31172: f64, t31382: f64, t31390: f64, t31451: f64, t4248: f64, t4254: f64, t49686: f64, t569: f64, t75485: f64, t75667: f64, t7732: f64, t7889: f64, t8307: f64, t8325: f64, t8393: f64, t8413: f64, t98484: f64, t98487: f64) -> f64 {
    let t118083 = 2.0_f64 * t1312 * t31157 * t1911 + 4.0_f64 * t2322 * t31382 + 4.0_f64 * t1312 * t31451 * t1453 + 2.0_f64 * t1312 * t118019 * t569 + 4.0_f64 * t4248 * t31161 + 2.0_f64 * t7889 * t31158 - 2.0_f64 * t4248 * t31172 - 2.0_f64 * t98484 * t2199 - 4.0_f64 * t98487 * t2199 - 4.0_f64 * t27123 * t8307 + 4.0_f64 * t13435 * t8413 - 2.0_f64 * t18163 * t8393 - 4.0_f64 * t4254 * t31390 - 2.0_f64 * t7732 * t31169 + 2.0_f64 * t75485 * t2201 + 4.0_f64 * t18227 * t8325 - 2.0_f64 * t49686 * t2199 - 4.0_f64 * t75667 * t2199 - 4.0_f64 * t13426 * t8307 + 2.0_f64 * t4248 * t31158;
    t118083
}
