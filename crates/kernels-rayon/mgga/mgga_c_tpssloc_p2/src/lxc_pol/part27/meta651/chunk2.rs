//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2266/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2266(t12571: f64, t608: f64, t33: f64, t46099: f64, t2244: f64, t3953: f64, t1865: f64, t22513: f64, t22516: f64, t22534: f64, t22551: f64, t26016: f64, t26028: f64, t6506: f64, t6510: f64, t7428: f64, t7442: f64, t7446: f64, t83725: f64, t83729: f64, t83738: f64) -> f64 {
    let t90114 = t12571 * t608;
    let t90121 = t46099 * t33;
    let t90132 = t3953 * t2244;
    let t90135 = -10.0_f64 / 3.0_f64 * t26016 * t83725 - 10.0_f64 / 3.0_f64 * t26016 * t83729 - 5.0_f64 / 3.0_f64 * t26016 * t83738 - 10.0_f64 / 3.0_f64 * t90114 * t22551 + t22534 * t7446 / 3.0_f64 + t22534 * t7442 / 3.0_f64 - t90121 * t1865 / 6.0_f64 - t26028 * t6506 / 3.0_f64 - t26028 * t6510 / 3.0_f64 - t7428 * t22513 / 6.0_f64 - t7428 * t22516 / 3.0_f64 + t90132 * t1865 / 3.0_f64;
    t90135
}
