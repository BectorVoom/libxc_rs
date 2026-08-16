//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 776/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk776(t1532: f64, t7046: f64, t133: f64, t594: f64, t1020: f64, t1773: f64, t2575: f64, t614: f64, t1790: f64, t2702: f64, t183: f64, t5389: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7047 = t7046 * t1532;
    let t7065 = t594 * t133;
    let t7070 = t1773 * t1020;
    let t7074 = t614 * t2575;
    let t7116 = t1790 * t2702;
    let t7123 = t5389 * t183;
    (t7047, t7065, t7070, t7074, t7116, t7123)
}
