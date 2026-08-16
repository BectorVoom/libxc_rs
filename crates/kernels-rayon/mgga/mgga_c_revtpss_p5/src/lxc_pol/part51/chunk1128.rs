//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1128/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1128(t651: f64, t7002: f64, t7883: f64, t27123: f64, t8461: f64, t27126: f64, t1583: f64, t7086: f64, t27383: f64, t198: f64, t8536: f64, t1940: f64, t2255: f64, t8494: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t125945 = t651 * t7883 * t7002;
    let t125948 = 2.0_f64 * t27123 * t8461;
    let t125950 = 2.0_f64 * t27126 * t8461;
    let t125961 = t1583 * t7086;
    let t125962 = t27383 * t125961;
    let t125968 = t198 * t8536;
    let t125976 = t1940 * t8494 * t2255;
    (t125945, t125948, t125950, t125961, t125962, t125968, t125976)
}
