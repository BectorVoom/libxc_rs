//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1286/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1286(t6562: f64, t82133: f64, t8547: f64, t7106: f64, t857: f64, t225: f64, t31362: f64, t23030: f64, t31405: f64, t31315: f64, t794: f64, t23012: f64, t8548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114811 = t31362 * t225;
    let t114814 = t23030 * t31405;
    let t114815 = 0.26044789391763585244e-1_f64 * t114814;
    let t114827 = t6562 * t794 * t31315;
    let t114864 = t23012 * t8548;
    (t114795, t114797, t114811, t114815, t114827, t114864)
}
