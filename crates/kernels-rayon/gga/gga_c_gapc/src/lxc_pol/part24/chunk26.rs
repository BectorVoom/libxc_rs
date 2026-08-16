//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 26/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk26(t76: f64, t79: f64, t19: f64, t2: f64, t20: f64, t5: f64, t60: f64, t7: f64) -> (f64, f64, f64, f64, f64) {
    let t80 = t76 * t79;
    let t83 = t19 * t20 * t2;
    let t84 = t5 * t60;
    let t85 = t83 * t84;
    let t87 = t7 * t7;
    (t80, t83, t84, t85, t87)
}
