//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 592/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk592(t448: f64, t995: f64, t459: f64, t2490: f64, t2494: f64, t2500: f64, t2504: f64, t34: f64, t38: f64, t445: f64, t454: f64, t974: f64, t991: f64) -> (f64, f64, f64) {
    let t2510 = t995 * t448;
    let t2513 = t995 * t459;
    let t2528 = -25.0_f64 / 9.0_f64 * t454 * t974 + 10.0_f64 / 9.0_f64 * t34 * t2490 + 5.0_f64 / 3.0_f64 * t34 * t2494 - 25.0_f64 / 9.0_f64 * t991 * t445 + 10.0_f64 / 9.0_f64 * t38 * t2500 - 5.0_f64 / 3.0_f64 * t38 * t2504;
    (t2510, t2513, t2528)
}
