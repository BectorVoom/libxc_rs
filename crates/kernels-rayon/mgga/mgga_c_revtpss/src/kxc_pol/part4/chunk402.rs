//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 402/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk402(t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t460: f64, t495: f64, t498: f64) -> (f64, f64) {
    let t1298 = 0.65854491829355115987e0_f64 * t1204 * t495 - 0.65854491829355115987e0_f64 * t1210 * t1215 + 0.65854491829355115987e0_f64 * t460 * t1271 - 0.65854491829355115987e0_f64 * t1274 * t1295;
    let t1300 = 1.0_f64 / t498;
    (t1298, t1300)
}
