//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 322/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk322(t1073: f64, t665: f64, t672: f64, t1066: f64, t208: f64, t218: f64, t219: f64, t1068: f64, t670: f64, t678: f64) -> (f64, f64, f64, f64, f64) {
    let t1074 = t665 * t1073;
    let t1077 = t672 * t1073;
    let t1079 = t208 * t1066;
    let t1081 = t218 * t219 * t1079;
    let t1083 = 0.1898925e1_f64 * t1074 - t670 + 0.8969e0_f64 * t1068 + 0.3071625e0_f64 * t1077 - t678 + 0.24647e0_f64 * t1081;
    (t1074, t1077, t1079, t1081, t1083)
}
