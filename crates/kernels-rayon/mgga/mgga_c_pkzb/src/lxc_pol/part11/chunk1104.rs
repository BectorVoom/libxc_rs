//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1104/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1104(t1070: f64, t5801: f64, t1095: f64, t1938: f64, t5830: f64, t5775: f64, t2848: f64, t5493: f64, t20716: f64, t20748: f64, t20751: f64, t1915: f64, t2793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20896 = t1070 * t5801;
    let t20905 = t1938 * t1095;
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20982 = t2848 * t5493;
    let t21055 = 0.20659e1_f64 * t20716;
    let t21058 = 0.104195e1_f64 * t20748;
    let t21059 = 0.104195e1_f64 * t20751;
    let t21087 = t2793 * t1915;
    (t20896, t20905, t20908, t20911, t20982, t21055, t21058, t21059, t21087)
}
