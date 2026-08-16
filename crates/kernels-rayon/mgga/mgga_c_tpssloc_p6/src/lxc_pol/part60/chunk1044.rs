//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1044/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1044(t22635: f64, t26331: f64, t31549: f64, t6330: f64, t115630: f64, t122390: f64, t122551: f64, t127434: f64, t127442: f64, t127445: f64, t127448: f64, t127455: f64, t127459: f64, t127463: f64, t128841: f64, t128855: f64, t128874: f64, t128882: f64, t1375: f64, t1378: f64, t20060: f64, t29299: f64, t29372: f64, t33320: f64, t5215: f64, t5321: f64, t6958: f64, t8637: f64) -> f64 {
    let t128894 = t26331 * t22635 * t31549 * t6330;
    let t128902 = -t1375 * t1378 * (t128841 + t128855 + t128874 + t128882) + 4.0_f64 * t5215 * t33320 + 2.0_f64 * t6958 * t29372 + 0.82246703342411321824e-2_f64 * t122390 - t127434 + t115630 - t127442 - 0.49348022005446793095e-1_f64 * t128894 - t127445 + 4.0_f64 * t5321 * t33320 - t127448 - t20060 * t8637 + t127455 - 0.82246703342411321824e-2_f64 * t122551 - 6.0_f64 * t6958 * t29299 + t127459 - t127463;
    t128902
}
