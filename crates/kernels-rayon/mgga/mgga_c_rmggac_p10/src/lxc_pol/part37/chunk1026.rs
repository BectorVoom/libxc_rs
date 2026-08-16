//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1026/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1026(t78596: f64, t1356: f64, t13957: f64, t43974: f64, t7879: f64, t884: f64, t9530: f64, t577: f64, t703: f64, t7933: f64, t7934: f64, t76547: f64) -> (f64, f64, f64, f64, f64) {
    let t78597 = 0.36366215538993788971e-1_f64 * t78596;
    let t78602 = 0.11974241701863808564e0_f64 * t1356 * t43974 * t13957;
    let t78605 = 0.11974241701863808564e0_f64 * t884 * t9530 * t7879;
    let t78608 = t7933 * t7934 * t577 * t703;
    let t78609 = 0.36021158228745895953e-3_f64 * t78608;
    let t78611 = 0.20496175532535769483e-3_f64 * t76547;
    (t78597, t78602, t78605, t78609, t78611)
}
