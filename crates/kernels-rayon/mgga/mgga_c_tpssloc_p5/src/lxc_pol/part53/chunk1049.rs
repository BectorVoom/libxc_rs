//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1049/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1049(t12461: f64, t8803: f64, t102344: f64, t117014: f64, t121004: f64, t121007: f64, t123368: f64, t124293: f64, t1458: f64, t2039: f64, t23938: f64, t26977: f64, t27170: f64, t27188: f64, t32235: f64, t33234: f64, t35233: f64, t4072: f64, t671: f64, t7042: f64, t7056: f64, t7801: f64, t92090: f64) -> (f64, f64) {
    let t124476 = t8803 * t12461;
    let t124531 = 4.0_f64 * t102344 * t2039 + 2.0_f64 * t117014 * t1458 + 4.0_f64 * t121004 * t2039 + 4.0_f64 * t121007 * t2039 + 2.0_f64 * t123368 * t671 + 2.0_f64 * t124293 * t1458 + 4.0_f64 * t2039 * t92090 + 4.0_f64 * t23938 * t7801 + 4.0_f64 * t26977 * t7801 + 4.0_f64 * t27170 * t7042 + 4.0_f64 * t27188 * t7056 + 2.0_f64 * t32235 * t4072 + 4.0_f64 * t33234 * t7056 + 4.0_f64 * t35233 * t7056;
    (t124476, t124531)
}
