//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1883/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1883(t1113: f64, t14753: f64, t136: f64, t14744: f64, t11265: f64, t1661: f64, t3271: f64, t11243: f64, t3270: f64, t4756: f64, t1102: f64, t3279: f64, t4748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14795 = t1113 * t14753;
    let t14796 = t136 * t14795;
    let t14798 = t1113 * t14744;
    let t14799 = t136 * t14798;
    let t14801 = t11265 * t1661;
    let t14802 = t14801 * t3271;
    let t14804 = t11243 * t1661;
    let t14805 = t14804 * t3271;
    let t14808 = t3270 * t4756;
    let t14809 = t14808 * t1102;
    let t14811 = t4748 * t3279;
    (t14795, t14796, t14798, t14799, t14801, t14802, t14804, t14805, t14808, t14809, t14811)
}
