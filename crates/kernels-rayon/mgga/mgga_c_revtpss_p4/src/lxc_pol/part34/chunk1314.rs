//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1314/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1314(t1501: f64, t5920: f64, t1936: f64, t30138: f64, t7741: f64, t30004: f64, t4248: f64, t22633: f64, t93: f64, t30143: f64, t7889: f64, t22589: f64, t94982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114378 = t1501 * t5920;
    let t114380 = 6.0_f64 * t114378 * t1936;
    let t114382 = 12.0_f64 * t30138 * t7741;
    let t114384 = 6.0_f64 * t4248 * t30004;
    let t114385 = t93 * t22633;
    let t114387 = 2.0_f64 * t114385 * t1936;
    let t114389 = 6.0_f64 * t30143 * t7741;
    let t114391 = 6.0_f64 * t7889 * t30004;
    let t114394 = t94982 * t22589;
    (t114378, t114380, t114382, t114384, t114387, t114389, t114391, t114394)
}
