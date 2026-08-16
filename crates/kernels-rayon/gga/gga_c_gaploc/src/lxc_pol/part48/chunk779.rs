//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 779/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk779(t15667: f64, t39636: f64, t1233: f64, t15665: f64, t15672: f64, t39635: f64, t92: f64, t39644: f64, t5345: f64, t5348: f64, t1692: f64, t2519: f64) -> (f64, f64, f64, f64) {
    let t40622 = t39636 * t15667;
    let t40627 = t15672 * t1233 * t39635 * t15665 * t92;
    let t40630 = t5345 * t39644 * t5348;
    let t40632 = t1692 * t2519;
    (t40622, t40627, t40630, t40632)
}
