//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1374/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1374(t33390: f64, t33394: f64, t33396: f64, t33402: f64, t33405: f64, t33409: f64, t33413: f64, t33417: f64, t33420: f64, t33407: f64, t36570: f64, t33427: f64) -> (f64, f64) {
    let t36571 = 0.67528199161846004232e-6_f64 * t33390;
    let t36572 = 0.99537768901660885081e-7_f64 * t33394;
    let t36573 = 0.10551281119038438161e-7_f64 * t33396;
    let t36574 = 0.44197102999375800018e-8_f64 * t33402;
    let t36575 = 0.42205124476153752644e-7_f64 * t33405;
    let t36577 = 0.10551281119038438161e-7_f64 * t33409;
    let t36578 = 0.337303223138432284e-8_f64 * t33413;
    let t36579 = 0.55331893559454114829e-8_f64 * t33417;
    let t36580 = 0.66295654499063700026e-7_f64 * t33420;
    let t36581 = -t36570 + t36571 - t36572 - t36573 + t36574 - t36575 - 0.3623181683912940217e-6_f64 * t33407 + t36577 + t36578 + t36579 - t36580;
    let t36585 = 0.11372686522837130914e-5_f64 * t33427;
    (t36581, t36585)
}
