//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3119/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119(t1164: f64, t15133: f64, t4874: f64, t11433: f64, t18910: f64, t1695: f64, t51810: f64, t64482: f64, t11126: f64, t6098: f64, t6102: f64, t18785: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64514 = 0.23392894490538584828e1_f64 * t1164 * t4874 * t15133;
    let t64517 = 0.17315859105681463759e2_f64 * t1164 * t18910 * t11433;
    let t64520 = 0.14035736694323150897e2_f64 * t51810 * t1695 * t64482;
    let t64522 = 0.11696447245269292414e1_f64 * t11126 * t6098;
    let t64524 = 0.5848223622634646207e0_f64 * t11126 * t6102;
    let t64525 = t3400 * t18785;
    (t64514, t64517, t64520, t64522, t64524, t64525)
}
