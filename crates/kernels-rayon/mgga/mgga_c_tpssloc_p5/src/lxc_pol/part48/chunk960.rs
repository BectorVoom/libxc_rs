//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 960/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk960(t23222: f64, t31366: f64, t6552: f64, t225: f64, t31362: f64, t23030: f64, t31405: f64, t23270: f64, t2379: f64, t25038: f64, t31337: f64, t31315: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t114808 = t6552 * t31366 * t23222;
    let t114811 = t31362 * t225;
    let t114814 = t23030 * t31405;
    let t114815 = 0.26044789391763585244e-1_f64 * t114814;
    let t114822 = t25038 * t23270 * t31337 * t2379;
    let t114827 = t6562 * t794 * t31315;
    (t114808, t114811, t114815, t114822, t114827)
}
