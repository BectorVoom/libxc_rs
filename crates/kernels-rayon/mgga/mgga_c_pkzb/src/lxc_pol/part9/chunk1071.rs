//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1071/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1071(t1548: f64, t1626: f64, t1485: f64, t1508: f64, t1531: f64, t1499: f64, t126: f64, t82: f64, t94: f64, t98: f64, t501: f64, t5175: f64) -> (f64, f64, f64, f64, f64) {
    let t16882 = t1548 * t1626;
    let t16886 = 0.12842595503380418954e1_f64 * t1531 * t1485 * t1508;
    let t16889 = 0.43374325201206959368e-1_f64 * t1531 * t1485 * t1499;
    let t16893 = 24.0_f64 * t82 * t94 * t98 * t126;
    let t16894 = t501 * t5175;
    (t16882, t16886, t16889, t16893, t16894)
}
