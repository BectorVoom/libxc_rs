//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1251/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251(t10876: f64, t21396: f64, t248: f64, t3101: f64, t1041: f64, t21138: f64, t3051: f64, t21134: f64, t14508: f64, t17667: f64, t17611: f64, t4641: f64) -> (f64, f64, f64, f64, f64) {
    let t70162 = t10876 * t248 * t3101 * t21396;
    let t70166 = t1041 * t248 * t3051 * t21138;
    let t70199 = t1041 * t248 * t3051 * t21134;
    let t70209 = t14508 * t17667;
    let t70214 = t4641 * t17611;
    (t70162, t70166, t70199, t70209, t70214)
}
