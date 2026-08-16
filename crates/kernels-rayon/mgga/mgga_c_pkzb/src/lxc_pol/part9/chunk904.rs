//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 904/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk904(t24: f64, t1003: f64, t5106: f64, t1651: f64, t8: f64, t1429: f64, t507: f64, t1652: f64, t1655: f64, t2548: f64, t2551: f64, t82: f64, t91: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t6782 = t5106 * t1003;
    let t6785 = t1651 * t8;
    let t6786 = t1429 * t507;
    let t6796 = piecewise3(t90, 0.0_f64, -8.0_f64 / 27.0_f64 * t6782 * t1652 - 16.0_f64 / 9.0_f64 * t6785 * t6786 + 4.0_f64 / 9.0_f64 * t2548 * t1655 - 8.0_f64 / 3.0_f64 * t91 * t1429 + 8.0_f64 * t2551 * t82);
    (t6782, t6785, t6786, t6796)
}
