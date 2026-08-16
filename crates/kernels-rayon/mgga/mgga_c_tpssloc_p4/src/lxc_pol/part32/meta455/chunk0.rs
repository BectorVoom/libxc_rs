//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1729/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1729(t23030: f64, t6643: f64, t131: f64, t244: f64, t209: f64, t1878: f64, t6612: f64, t835: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23031 = t23030 * t6643;
    let t23032 = 0.26044789391763585244e-1_f64 * t23031;
    let t23033 = t244 * t131;
    let t23034 = t23033 * t209;
    let t23035 = t1878 * t23034;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    (t23032, t23033, t23034, t23035, t23040, t23041)
}
