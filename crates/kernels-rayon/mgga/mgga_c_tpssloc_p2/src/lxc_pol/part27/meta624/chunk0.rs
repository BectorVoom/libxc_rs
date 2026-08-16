//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2104/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104(t12521: f64, t7467: f64, t81440: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64, t2332: f64, t81446: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86582 = 0.135e2_f64 * t12521 * t7467;
    let t86583 = 22.0_f64 / 9.0_f64 * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86589 = 4.0_f64 / 3.0_f64 * t86588;
    let t86590 = t22470 * t4067;
    let t86591 = 2.0_f64 / 3.0_f64 * t86590;
    let t86592 = t1453 * t2332;
    let t86593 = t81446 * t86592;
    let t86595 = t4067 * t666;
    (t86582, t86583, t86586, t86589, t86591, t86593, t86595)
}
