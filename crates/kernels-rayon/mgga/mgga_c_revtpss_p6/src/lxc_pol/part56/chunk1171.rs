//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1171/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1171(t5: f64, t131276: f64, t131318: f64, t117: f64, t125510: f64, t125512: f64, t125514: f64, t125521: f64, t125525: f64, t129326: f64, t129328: f64, t129332: f64, t129335: f64, t129339: f64, t129342: f64, t129344: f64, t1310: f64, t131234: f64, t1911: f64, t33343: f64, t33381: f64, t34874: f64, t35014: f64, t4248: f64, t508: f64, t649: f64, t651: f64, t670: f64, t671: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t131320 = piecewise3(t8, 0.0_f64, t131276 + t131318);
    let t131321 = t131320 * t117;
    let t131331 = -2.0_f64 * t35014 * t651 * t670 - t1310 * t34874 - 2.0_f64 * t131234 * t671 - t131321 * t508 + t1911 * t33381 - 2.0_f64 * t33343 * t4248 - t35014 * t649 + t125510 + t125512 - t125514 - t125521 - t125525 - 4.0_f64 * t129326 - 4.0_f64 * t129328 - 4.0_f64 * t129332 - 4.0_f64 * t129335 - 2.0_f64 * t129339 + 6.0_f64 * t129342 + 6.0_f64 * t129344;
    (t131321, t131331)
}
