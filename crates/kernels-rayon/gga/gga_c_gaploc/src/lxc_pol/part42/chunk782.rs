//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 782/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk782(t18313: f64, t31119: f64, t3394: f64, t9273: f64, t35180: f64, t9562: f64, t10256: f64, t30830: f64, t913: f64, t12957: f64, t31356: f64, t35216: f64, t9287: f64) -> (f64, f64, f64, f64, f64) {
    let t41660 = t31119 * t18313 * t3394 * t9273;
    let t41666 = t35180 * t9562;
    let t41669 = t30830 * t913 * t10256;
    let t41674 = t31356 * t12957;
    let t41676 = t35216 * t9287;
    (t41660, t41666, t41669, t41674, t41676)
}
