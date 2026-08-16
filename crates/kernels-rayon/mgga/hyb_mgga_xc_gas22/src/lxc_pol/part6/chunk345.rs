//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 345/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk345(t43: f64, t1179: f64, t559: f64, t575: f64, t578: f64, t570: f64, t572: f64) -> (f64, f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t1184 = t559 * t1179;
    let t1189 = t575 * t578 * t1179;
    let t1192 = -t572 * t1189 / 54.0_f64 - t570 / 54.0_f64;
    let t1193 = piecewise3(t45, t1192, 0.0_f64);
    (t1184, t1189, t1192, t1193)
}
