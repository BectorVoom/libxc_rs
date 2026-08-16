//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1999/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1999(t15708: f64, t4728: f64, t3578: f64, t1735: f64, t3243: f64, t11668: f64, t1744: f64, t3540: f64, t1731: f64, t1222: f64, t4961: f64, t1743: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15709 = t4728 * t15708;
    let t15710 = t3578 * t15709;
    let t15713 = t1735 * t3243;
    let t15714 = t11668 * t15713;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / 432.0_f64;
    let t15723 = t3566 * t1743;
    (t15709, t15710, t15713, t15714, t15717, t15719, t15722, t15723)
}
