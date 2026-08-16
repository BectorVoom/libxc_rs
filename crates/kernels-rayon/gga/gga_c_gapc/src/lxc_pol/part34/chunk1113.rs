//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1113/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1113(t5247: f64, t681: f64, t9261: f64, t134: f64, t203: f64, t5700: f64, t137: f64, t1672: f64, t154: f64, t3954: f64, t26995: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27063 = t5247 * t681 * t9261;
    let t27144 = t203 * t134;
    let t27145 = t27144 * t5700;
    let t27149 = t1672 * t137;
    let t27286 = t154 * t3954;
    let t27290 = t26995 * t5544;
    (t27063, t27144, t27145, t27149, t27286, t27290)
}
