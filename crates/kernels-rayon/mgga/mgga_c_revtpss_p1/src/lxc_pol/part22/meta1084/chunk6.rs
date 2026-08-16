//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3931/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3931(t114: f64, t75532: f64, t75655: f64, t4245: f64, t670: f64, t10416: f64, t1312: f64, t13426: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t18227: f64, t18245: f64, t21881: f64, t2322: f64, t2371: f64, t27123: f64, t4248: f64, t4292: f64, t49686: f64, t5523: f64, t5920: f64, t60650: f64, t60656: f64, t61010: f64, t75439: f64, t75485: f64, t75494: f64, t7889: f64, t93: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t75657 = piecewise3(t115, 0.0_f64, t75532 + t75655);
    let t75667 = t4245 * t670;
    let t75672 = 2.0_f64 * t10416 * t5920 + 2.0_f64 * t1312 * t75657 + 8.0_f64 * t13426 * t4292 + 4.0_f64 * t13435 * t5920 + 2.0_f64 * t13440 * t5920 + 4.0_f64 * t13514 * t4248 + 4.0_f64 * t13514 * t7889 + 4.0_f64 * t1518 * t49686 + 4.0_f64 * t1518 * t75485 + 8.0_f64 * t1518 * t75667 + 8.0_f64 * t18227 * t4292 + 2.0_f64 * t18245 * t2371 + 4.0_f64 * t21881 * t2322 + 4.0_f64 * t21881 * t5523 + 8.0_f64 * t27123 * t4292 + 4.0_f64 * t670 * t75439 + 4.0_f64 * t75494 * t93 + 2.0_f64 * t60650 + 2.0_f64 * t60656 + t61010;
    (t75657, t75667, t75672)
}
