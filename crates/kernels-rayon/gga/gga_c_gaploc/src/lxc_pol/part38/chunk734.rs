//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 734/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk734(t1564: f64, t40: f64, t6509: f64, t9439: f64, t9448: f64, t585: f64, t9419: f64, t129: f64, t15481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20550 = t40 * t1564;
    let t20551 = t20550 * t6509;
    let t20556 = t9439 * t6509;
    let t20561 = t9448 * t6509;
    let t20669 = t585 * t9419;
    let t20671 = t15481 * t129;
    (t20550, t20551, t20556, t20561, t20669, t20671)
}
