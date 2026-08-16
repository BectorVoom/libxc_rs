//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1197/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1197(t3262: f64, t3574: f64, t38739: f64, t3472: f64, t40397: f64, t3579: f64, t39032: f64, t12042: f64, t37271: f64, t3465: f64, t39327: f64, t38771: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41280 = 3.0_f64 / 4.0_f64 * t3262 * t38739 * t3574;
    let t41283 = 15.0_f64 / 16.0_f64 * t3262 * t3472 * t40397;
    let t41285 = t3579 * t39032 / 2.0_f64;
    let t41286 = t37271 * t12042;
    let t41289 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t39327;
    let t41291 = 5.0_f64 / 8.0_f64 * t3579 * t38771;
    (t41280, t41283, t41285, t41286, t41289, t41291)
}
