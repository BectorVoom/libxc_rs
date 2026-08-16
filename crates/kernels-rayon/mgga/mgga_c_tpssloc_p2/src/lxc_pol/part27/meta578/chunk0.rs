//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2027/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2027(t1354: f64, t80991: f64, t1336: f64, t22759: f64, t835: f64, t3795: f64, t22765: f64, t3853: f64, t22704: f64, t22898: f64, t80798: f64, t12248: f64, t6604: f64) -> (f64, f64, f64, f64, f64) {
    let t80992 = t80991 * t1354;
    let t80997 = t1336 * t22759 * t835;
    let t80998 = t80997 * t3795;
    let t81007 = t22765 * t3853;
    let t81022 = t22704 * t80798 * t22898;
    let t81027 = t6604 * t12248;
    (t80992, t80998, t81007, t81022, t81027)
}
