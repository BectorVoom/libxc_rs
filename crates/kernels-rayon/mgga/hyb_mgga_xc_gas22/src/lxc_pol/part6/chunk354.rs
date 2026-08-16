//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 354/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk354(t143: f64, t1252: f64, t712: f64, t716: f64, t720: f64, t724: f64, t728: f64, t732: f64, t736: f64, t1251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t1255 = t712 * t1252;
    let t1257 = t716 * t1252;
    let t1259 = t720 * t1252;
    let t1261 = t724 * t1252;
    let t1263 = t728 * t1252;
    let t1265 = t732 * t1252;
    let t1267 = t736 * t1252;
    let t1270 = piecewise3(t145, 0.0_f64, t1251);
    (t1255, t1257, t1259, t1261, t1263, t1265, t1267, t1270)
}
