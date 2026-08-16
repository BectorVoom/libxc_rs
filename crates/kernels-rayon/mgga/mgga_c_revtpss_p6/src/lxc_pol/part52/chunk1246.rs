//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1246/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1246(t5: f64, t128422: f64, t128474: f64, t117: f64, t125385: f64, t125387: f64, t125389: f64, t125391: f64, t128367: f64, t32176: f64, t32178: f64, t33644: f64, t33646: f64, t8564: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t128476 = piecewise3(t8, 0.0_f64, t128422 + t128474);
    let t128477 = t128476 * t117;
    let t128478 = t128367 + t33644 + t33646 + t128477 + t8564 + t32176 + t32178 + t125385 + t125387 + t125389 + t125391;
    (t128477, t128478)
}
