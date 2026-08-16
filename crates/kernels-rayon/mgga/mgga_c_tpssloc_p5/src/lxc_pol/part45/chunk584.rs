//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 584/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk584(t6785: f64, t884: f64, t6784: f64, t1949: f64, t986: f64, t334: f64, t371: f64, t38: f64, t131: f64, t350: f64, t1009: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6786 = t6785 * t884;
    let t6787 = t6784 * t6786;
    let t6790 = t986 * t1949;
    let t6793 = t371 * t334;
    let t6794 = 1.0_f64 / t6793;
    let t6795 = t38 * t6794;
    let t6796 = t6795 * t131;
    let t6797 = t6796 * t350;
    let t6798 = t344 * t1009;
    (t6786, t6787, t6790, t6794, t6795, t6796, t6797, t6798)
}
