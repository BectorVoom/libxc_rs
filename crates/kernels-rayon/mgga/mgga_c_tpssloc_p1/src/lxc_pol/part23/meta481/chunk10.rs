//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1449/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449(t11692: f64, t1748: f64, t18395: f64, t19047: f64, t22208: f64, t22246: f64, t22258: f64, t22314: f64, t3578: f64, t5005: f64, t5019: f64, t5024: f64, t53083: f64, t6221: f64, t65528: f64, t72223: f64, t72225: f64, t72229: f64, t72248: f64, t72251: f64, t72253: f64, t72384: f64, t72767: f64) -> f64 {
    let t78713 = 19.0_f64 / 216.0_f64 * t72223 - t5019 * t22246 / 144.0_f64 + 5.0_f64 / 1728.0_f64 * t72225 + t72229 / 192.0_f64 - 19.0_f64 / 216.0_f64 * t72384 * t1748 + t19047 * t6221 / 512.0_f64 + 5.0_f64 / 243.0_f64 * t5024 * t22208 + t11692 * t3578 * t72767 * t18395 / 384.0_f64 - t72248 / 384.0_f64 - t65528 / 2304.0_f64 + t72251 / 54.0_f64 + t72253 / 54.0_f64 + t53083 * t22314 / 24.0_f64 - t5005 * t22258 / 192.0_f64;
    t78713
}
