//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1730/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1730(t1462: f64, t9912: f64, t9871: f64, t2427: f64, t4101: f64, t9880: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13102 = 4.0_f64 * t9912 * t1462;
    let t13103 = 0.4883052614935078681e-3_f64 * t9871;
    let t13105 = 8.0_f64 * t2427 * t4101;
    let t13106 = 0.21687162600603479684e-1_f64 * t9880;
    let t13107 = t4199 * t2528;
    let t13108 = 0.17315859105681463759e2_f64 * t13107;
    let t13109 = t4211 * t2663;
    let t13110 = 0.24415263074675393405e-3_f64 * t13109;
    let t13111 = t13102 + t13103 + t9793 + t9797 - t9876 + t13105 - t9820 - t9824 + t13106 - t9884 + t9887 + t9890 - t13108 - t9894 + t13110;
    (t13102, t13103, t13105, t13106, t13108, t13110, t13111)
}
