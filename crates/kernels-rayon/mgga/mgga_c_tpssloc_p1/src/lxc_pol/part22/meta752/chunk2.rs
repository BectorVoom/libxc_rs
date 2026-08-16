//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2527/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527(t423: f64, t71162: f64, t71214: f64, t1157: f64, t1164: f64, t21938: f64, t3375: f64, t1254: f64, t19270: f64, t4700: f64, t5091: f64, t71095: f64, t71097: f64, t71101: f64, t71106: f64, t71109: f64, t71112: f64, t71114: f64, t71118: f64) -> (f64, f64, f64) {
    let t71217 = 0.621814e-1_f64 * (t71162 + t71214) * t423;
    let t71221 = 0.11696447245269292414e1_f64 * t1164 * t3375 * t21938 * t1157;
    let t71222 = -t1254 * t4700 * t71101 + 6.0_f64 * t19270 * t4700 * t5091 + t71095 - t71097 + t71106 - t71109 - t71112 + t71114 + t71118 - t71217 + t71221;
    (t71217, t71221, t71222)
}
