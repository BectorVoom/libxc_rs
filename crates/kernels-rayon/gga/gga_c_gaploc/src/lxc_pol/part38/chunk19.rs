//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 19/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk19(t6: f64, t78: f64, t77: f64, t19: f64, t2: f64, t20: f64, t22: f64, t7: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t79 = t6 * t78;
    let t80 = t77 * t79;
    let t83 = t19 * t20 * t2;
    let t85 = 1.0_f64 / t22 / t7;
    let t86 = t5 * t85;
    let t87 = t83 * t86;
    let t89 = t7 * t7;
    (t79, t80, t83, t86, t87, t89)
}
