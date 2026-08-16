//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1048/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1048(t4034: f64, t7468: f64, t1266: f64, t7467: f64, t652: f64, t6876: f64, t7756: f64, t645: f64, t72: f64, t7431: f64, t1437: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26002 = 2.0_f64 * t4034 * t7468;
    let t26003 = t1266 * t7467;
    let t26005 = 2.0_f64 * t652 * t26003;
    let t26006 = t6876 * t7756;
    let t26009 = t72 * t7431 * t645;
    let t26012 = t1864 * t1437;
    (t26002, t26003, t26005, t26006, t26009, t26012)
}
