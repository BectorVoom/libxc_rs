//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2607/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607(t15740: f64, t18371: f64, t1222: f64, t22175: f64, t1090: f64, t11728: f64, t15569: f64, t18300: f64, t18383: f64, t18946: f64, t22312: f64, t3578: f64, t45114: f64, t4582: f64, t66052: f64, t66054: f64, t66057: f64, t66073: f64, t66076: f64, t66079: f64, t66084: f64, t66092: f64) -> f64 {
    let t72727 = t15740 * t18371;
    let t72733 = t22175 * t1222;
    let t72735 = -3.0_f64 / 512.0_f64 * t11728 * t4582 * t18300 * t18946 - t66052 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t66054 - t66057 / 324.0_f64 + t66073 / 2304.0_f64 - t66076 / 1152.0_f64 - t66079 / 1152.0_f64 + t15569 * t18383 / 288.0_f64 - t66084 / 384.0_f64 + t66092 / 384.0_f64 - t72727 / 1152.0_f64 + t45114 * t3578 * t22312 * t1090 / 768.0_f64 - 209.0_f64 / 3888.0_f64 * t72733;
    t72735
}
