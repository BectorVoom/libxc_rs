//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 521/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk521(t1234: f64, t136: f64, t137: f64, t9102: f64, t4074: f64, t4077: f64, t4082: f64, t4085: f64, t2282: f64, t3101: f64, t3106: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9103 = t1234 * t136;
    let t9105 = 1.0_f64 / t137 / t9103;
    let t9106 = t9102 * t9105;
    let t9108 = t9106 * t4074 * t4077;
    let t9111 = t4082 * t9106 * t4085;
    let t9113 = t3101 * t2282;
    let t9115 = t3106 * t467;
    (t9105, t9106, t9108, t9111, t9113, t9115)
}
