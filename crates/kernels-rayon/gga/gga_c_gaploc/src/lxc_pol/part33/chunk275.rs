//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 275/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk275(t386: f64, t90: f64, t71: f64, t64: f64, t397: f64, t110: f64, t19: f64, t67: f64, t20: f64, t5: f64, t163: f64, t1: f64, t341: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1088 = t90 * t386;
    let t1091 = t71 * t71;
    let t1092 = 1.0_f64 / t1091;
    let t1093 = t64 * t1092;
    let t1094 = t397 * t397;
    let t1097 = 1.0_f64 / t110;
    let t1099 = t1097 * t67 * t19;
    let t1100 = t20 * t5;
    let t1101 = t1100 * t163;
    let t1105 = t341 * t394 * t1;
    (t1088, t1093, t1094, t1097, t1099, t1101, t1105)
}
