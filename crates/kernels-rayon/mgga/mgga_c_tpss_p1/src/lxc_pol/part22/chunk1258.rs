//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1258/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1258(t20177: f64, t20216: f64, t509: f64, t1270: f64, t13965: f64, t18690: f64, t4525: f64, t5936: f64, t508: f64, t6435: f64, t5709: f64, t10292: f64, t5784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20217 = t20177 + t20216;
    let t20218 = t509 * t20217;
    let t20219 = t20218 * t1270;
    let t20221 = t18690 * t13965;
    let t20224 = t5936 * t4525;
    let t20226 = t508 * t6435;
    let t20227 = t20226 * t5709;
    let t20246 = t10292 * t5784;
    (t20217, t20218, t20219, t20221, t20224, t20226, t20227, t20246)
}
