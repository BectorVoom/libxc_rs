//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1207/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1207(t17326: f64, t2751: f64, t5734: f64, t7269: f64, t7272: f64, t17329: f64, t7275: f64, t17655: f64, t2787: f64, t5771: f64, t7279: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21004 = 6.0_f64 * t17326 * t2751;
    let t21006 = 12.0_f64 * t5734 * t7269;
    let t21008 = 6.0_f64 * t5734 * t7272;
    let t21010 = 0.28947563097646563121e3_f64 * t17329 * t7275;
    let t21012 = 0.48245938496077605201e2_f64 * t17655 * t2787;
    let t21014 = 0.96491876992155210402e2_f64 * t5771 * t7279;
    let t21016 = 0.48245938496077605201e2_f64 * t5771 * t7282;
    (t21004, t21006, t21008, t21010, t21012, t21014, t21016)
}
