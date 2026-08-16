//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1132/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1132(t14402: f64, t7704: f64, t2894: f64, t2811: f64, t330: f64, t1008: f64, t1646: f64, t4947: f64, t26679: f64, t4547: f64, t283: f64, t4981: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27815 = t7704 * t14402;
    let t27816 = t2894 * t27815;
    let t27819 = t2811 * t330;
    let t27820 = t1646 * t1008;
    let t27821 = t27819 * t27820;
    let t27822 = t4947 * t27821;
    let t27825 = t26679 * t4547;
    let t27826 = t4947 * t27825;
    let t27832 = t4981 * t283 * t990;
    (t27815, t27816, t27819, t27821, t27822, t27825, t27826, t27832)
}
