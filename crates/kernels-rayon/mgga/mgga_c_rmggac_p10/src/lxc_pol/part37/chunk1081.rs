//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1081/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1081(t75572: f64, t75590: f64, t75593: f64, t69871: f64, t73382: f64, t73383: f64, t75596: f64, t75602: f64, t77664: f64, t77665: f64, t77666: f64, t77669: f64, t77670: f64, t77672: f64, t77677: f64, t77679: f64, t77681: f64) -> f64 {
    let t80264 = 0.15372131649401827112e-4_f64 * t75572;
    let t80265 = 0.17347588262831798124e-4_f64 * t75590;
    let t80266 = 0.17347588262831798124e-4_f64 * t75593;
    let t80268 = -t73382 - t73383 - t80264 - t77664 - t77665 + t77666 + t77669 - t77670 - t77672 + t80265 + t80266 + t69871 - 0.81756761766873046873e-6_f64 * t75596 + t77677 + t75602 - t77679 - t77681;
    t80268
}
