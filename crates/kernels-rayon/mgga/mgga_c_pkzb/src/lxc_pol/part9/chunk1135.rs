//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1135/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1135(t24: f64, t1429: f64, t1652: f64, t1655: f64, t507: f64, t82: f64, t2551: f64, t500: f64, t1003: f64, t16250: f64, t1651: f64, t2548: f64, t5106: f64, t5107: f64, t5113: f64, t6782: f64, t6785: f64, t8: f64, t91: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t19660 = t1429 * t1652;
    let t19663 = t507 * t1655;
    let t19669 = t82 * t507;
    let t19672 = t1429 * t1655;
    let t19680 = 32.0_f64 * t2551 * t500;
    let t19682 = piecewise3(t90, 0.0_f64, 40.0_f64 / 81.0_f64 * t16250 * t1003 * t5107 + 16.0_f64 / 9.0_f64 * t5106 * t8 * t19660 - 8.0_f64 / 9.0_f64 * t6782 * t19663 - 8.0_f64 / 3.0_f64 * t1651 * t1429 * t507 + 8.0_f64 * t6785 * t19669 - 8.0_f64 / 3.0_f64 * t6785 * t19672 + 4.0_f64 / 9.0_f64 * t2548 * t5113 + 16.0_f64 * t91 * t82 - t19680);
    (t19660, t19663, t19669, t19672, t19682)
}
