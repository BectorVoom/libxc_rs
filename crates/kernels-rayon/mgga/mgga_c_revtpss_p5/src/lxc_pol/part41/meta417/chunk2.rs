//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1471/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1471(t31292: f64, t569: f64, t2178: f64, t5517: f64, t1312: f64, t13426: f64, t18227: f64, t2179: f64, t2181: f64, t2322: f64, t27123: f64, t28219: f64, t31248: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t7732: f64, t7889: f64, t8274: f64, t8278: f64, t8280: f64, t8353: f64, t8367: f64) -> (f64, f64, f64) {
    let t31293 = t31292 * t569;
    let t31299 = t5517 * t2178;
    let t31303 = t1312 * t31248 + t1312 * t31293 - t13426 * t2179 - t18227 * t2179 + t2181 * t27123 + t2181 * t28219 - t2322 * t8353 + t2322 * t8367 - t31299 * t651 - t4248 * t8274 + t4248 * t8280 - t4254 * t8353 + t5523 * t8367 - t7732 * t8274 + t7889 * t8278 + t7889 * t8280;
    (t31293, t31299, t31303)
}
