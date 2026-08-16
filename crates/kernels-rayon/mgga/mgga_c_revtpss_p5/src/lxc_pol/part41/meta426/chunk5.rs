//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1491/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1491(t13426: f64, t18227: f64, t18245: f64, t2179: f64, t2181: f64, t27123: f64, t27126: f64, t28219: f64, t31248: f64, t31299: f64, t31309: f64, t31318: f64, t31324: f64, t4248: f64, t651: f64, t6765: f64, t75439: f64, t7732: f64, t7889: f64, t8254: f64, t8273: f64, t8278: f64, t8353: f64, t8363: f64, t8369: f64, t85360: f64) -> f64 {
    let t118456 = -2.0_f64 * t651 * t6765 * t8273 - 4.0_f64 * t13426 * t8363 - 4.0_f64 * t18227 * t8363 - 2.0_f64 * t18245 * t8254 + 2.0_f64 * t18245 * t8278 - 2.0_f64 * t2179 * t75439 - 2.0_f64 * t2179 * t85360 + 2.0_f64 * t2181 * t85360 - 4.0_f64 * t27123 * t8353 - 4.0_f64 * t27123 * t8363 + 4.0_f64 * t27123 * t8369 - 4.0_f64 * t27126 * t8353 - 4.0_f64 * t27126 * t8363 + 4.0_f64 * t28219 * t8369 + 4.0_f64 * t31248 * t4248 - 4.0_f64 * t31299 * t7732 + 4.0_f64 * t31309 * t7889 - 4.0_f64 * t31318 * t4248 - 4.0_f64 * t31318 * t7732 + 4.0_f64 * t31324 * t4248;
    t118456
}
