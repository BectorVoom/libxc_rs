//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1182/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1182(t2575: f64, t568: f64, t1058: f64, t17245: f64, t1535: f64, t16923: f64, t1816: f64, t20350: f64, t20351: f64, t20352: f64, t20354: f64, t20357: f64, t20359: f64, t2536: f64, t2718: f64, t5082: f64, t5162: f64, t6758: f64, t6806: f64, t7181: f64) -> (f64, f64) {
    let t20592 = t2575 * t568;
    let t20603 = t1058 * t17245;
    let t20610 = -18.0_f64 * t1535 * t6806 * t7181 - 3.0_f64 * t1816 * t2536 * t7181 - 6.0_f64 * t20603 * t2536 * t5162 - 18.0_f64 * t2718 * t5082 * t6758 - t16923 - t20350 + t20351 - t20352 - t20354 - t20357 - t20359;
    (t20592, t20610)
}
