//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1498/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1498(t114: f64, t117477: f64, t117517: f64, t117560: f64, t117572: f64, t101522: f64, t1312: f64, t13435: f64, t18153: f64, t18163: f64, t1911: f64, t2178: f64, t2181: f64, t27123: f64, t28219: f64, t31066: f64, t31067: f64, t31070: f64, t31084: f64, t31309: f64, t31318: f64, t31324: f64, t4151: f64, t4248: f64, t4254: f64, t508: f64, t5523: f64, t651: f64, t7889: f64, t8278: f64, t8280: f64, t8362: f64, t8363: f64, t8369: f64, t98484: f64, t98487: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t117575 = piecewise3(t115, 0.0_f64, t117477 + t117517 + t117560 + t117572);
    let t117579 = 2.0_f64 * t1312 * t31066 * t1911 + 4.0_f64 * t28219 * t8280 + 2.0_f64 * t7889 * t31084 + 2.0_f64 * t98484 * t2181 + 4.0_f64 * t98487 * t2181 + 4.0_f64 * t27123 * t8278 - 2.0_f64 * t18163 * t8363 - 4.0_f64 * t4254 * t31318 + 4.0_f64 * t13435 * t8369 + 4.0_f64 * t5523 * t31324 + 2.0_f64 * t101522 * t2181 + 4.0_f64 * t28219 * t8278 + 2.0_f64 * t4248 * t31084 + 2.0_f64 * t4248 * t31067 + 2.0_f64 * t1312 * t8362 * t4151 + 2.0_f64 * t7889 * t31067 + 4.0_f64 * t7889 * t31070 - 2.0_f64 * t651 * t18153 * t2178 + 4.0_f64 * t5523 * t31309 - 2.0_f64 * t651 * t508 * t117575;
    (t117575, t117579)
}
