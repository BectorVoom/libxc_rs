//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 775/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk775(t12557: f64, t2518: f64, t135: f64, t1691: f64, t458: f64, t5337: f64, t9105: f64, t15667: f64, t39636: f64, t1233: f64, t15665: f64, t15672: f64, t39635: f64, t92: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t40614 = t2518 * t12557;
    let t40620 = t9105 * t5337 * pi * t1691 * t135 * t458;
    let t40622 = t39636 * t15667;
    let t40627 = t15672 * t1233 * t39635 * t15665 * t92;
    (t40614, t40620, t40622, t40627)
}
