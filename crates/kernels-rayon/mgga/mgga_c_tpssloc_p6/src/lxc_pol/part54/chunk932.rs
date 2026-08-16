//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 932/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk932(t23030: f64, t6643: f64, t131: f64, t244: f64, t6612: f64, t835: f64, t812: f64, t831: f64, t2627: f64, t59: f64, t2617: f64, t6613: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23031 = t23030 * t6643;
    let t23032 = 0.26044789391763585244e-1_f64 * t23031;
    let t23033 = t244 * t131;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23042 = t23041 * t831;
    let t23043 = 7.0_f64 / 1152.0_f64 * t23042;
    let t23046 = t2627 * t59;
    let t23053 = t2617 * t6613;
    (t23031, t23032, t23033, t23041, t23042, t23043, t23046, t23053)
}
