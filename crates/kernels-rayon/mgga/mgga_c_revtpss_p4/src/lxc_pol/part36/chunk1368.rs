//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1368/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1368(t114216: f64, t114221: f64, t114230: f64, t114238: f64, t114407: f64, t114410: f64, t114415: f64, t114417: f64, t114419: f64, t114421: f64, t114427: f64, t1502: f64, t1518: f64, t1843: f64, t2127: f64, t25043: f64, t25045: f64, t30724: f64, t30944: f64, t30951: f64, t34446: f64, t4248: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t7586: f64, t8152: f64, t8233: f64) -> f64 {
    let t116722 = -6.0_f64 * t1518 * t30944 * t651 - 6.0_f64 * t5920 * t651 * t8233 - 3.0_f64 * t1502 * t30944 - 6.0_f64 * t1843 * t30724 - t2127 * t25043 - 6.0_f64 * t25045 * t7586 - 6.0_f64 * t30951 * t4248 - 6.0_f64 * t34446 * t5921 - 3.0_f64 * t6765 * t8152 - t114216 + t114221 - t114230 - t114238 - t114407 - t114410 - t114415 - t114417 - t114419 - t114421 + t114427;
    t116722
}
