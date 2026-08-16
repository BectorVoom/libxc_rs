//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1348/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1348(t2234: f64, t3356: f64, t8853: f64, t10658: f64, t2228: f64, t6562: f64, t20829: f64, t20832: f64, t2189: f64, t4113: f64, t10648: f64, t6497: f64) -> (f64, f64, f64, f64) {
    let t29411 = 0.32163958997385070134e2_f64 * t2234 * t3356 * t8853;
    let t29414 = 0.51726012919273400301e3_f64 * t6562 * t10658 * t2228;
    let t29418 = 0.24955700379505800916e5_f64 * t20829 * t4113 * t20832 * t2189;
    let t29420 = 4.0_f64 * t6497 * t10648;
    (t29411, t29414, t29418, t29420)
}
