//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 783/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk783(t3338: f64, t4130: f64, t2482: f64, t9272: f64, t12960: f64, t1537: f64, t34890: f64, t6583: f64, t9537: f64, t10473: f64, t9263: f64, t10469: f64, t9267: f64) -> (f64, f64, f64, f64, f64) {
    let t41590 = t4130 * t3338;
    let t41592 = t9272 * t41590 * t2482;
    let t41594 = t1537 * t12960;
    let t41606 = t6583 * t34890 * t9537;
    let t41609 = t9263 * t10473 * t2482;
    let t41612 = t9267 * t10469 * t2482;
    (t41592, t41594, t41606, t41609, t41612)
}
