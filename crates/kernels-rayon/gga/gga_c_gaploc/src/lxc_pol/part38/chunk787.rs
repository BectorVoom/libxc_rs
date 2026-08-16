//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 787/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk787(t10256: f64, t30830: f64, t913: f64, t2482: f64, t3358: f64, t9263: f64, t12957: f64, t31356: f64, t35216: f64, t9287: f64, t2792: f64, t3177: f64) -> (f64, f64, f64, f64, f64) {
    let t41669 = t30830 * t913 * t10256;
    let t41672 = t9263 * t3358 * t2482;
    let t41674 = t31356 * t12957;
    let t41676 = t35216 * t9287;
    let t41683 = t9263 * t2792 * t3177;
    (t41669, t41672, t41674, t41676, t41683)
}
