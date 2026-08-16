//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2004/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2004(t10336: f64, t1920: f64, t1922: f64, t1049: f64, t23592: f64, t10164: f64, t225: f64, t1921: f64, t6733: f64, t3034: f64, t336: f64, t131: f64, t350: f64, t38: f64) -> (f64, f64, f64, f64, f64) {
    let t82436 = 0.30461741978670859935e-2_f64 * t1920 * t10336 * t1922;
    let t82469 = t23592 * t1049;
    let t82481 = t225 * t10164;
    let t82502 = t6733 * t1921;
    let t82510 = 1.0_f64 / t3034 / t336;
    let t82513 = t38 * t82510 * t131 * t350;
    (t82436, t82469, t82481, t82502, t82513)
}
