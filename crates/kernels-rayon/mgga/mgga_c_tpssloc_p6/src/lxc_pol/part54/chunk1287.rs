//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1287/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1287(t23030: f64, t31405: f64, t31315: f64, t6562: f64, t794: f64, t23012: f64, t8548: f64, t214: f64, t7084: f64, t31329: f64, t6547: f64, t31319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114814 = t23030 * t31405;
    let t114815 = 0.26044789391763585244e-1_f64 * t114814;
    let t114827 = t6562 * t794 * t31315;
    let t114864 = t23012 * t8548;
    let t114865 = 0.63969658155208805863e-1_f64 * t114864;
    let t114866 = t214 * t7084;
    let t114882 = t6547 * t31329;
    let t114891 = t23030 * t31319;
    (t114815, t114827, t114865, t114866, t114882, t114891)
}
