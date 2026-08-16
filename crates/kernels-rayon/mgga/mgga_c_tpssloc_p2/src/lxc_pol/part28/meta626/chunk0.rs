//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1952/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952(t6875: f64, t8944: f64, t1845: f64, t3698: f64, t3734: f64, t12813: f64, t89: f64, t27240: f64, t580: f64, t1395: f64, t7961: f64, t1851: f64, t7240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91669 = t6875 * t8944;
    let t91687 = t1845 * t3698;
    let t91695 = t1845 * t3734;
    let t91753 = t89 * t12813;
    let t91830 = 2.0_f64 * t27240 * t580;
    let t91832 = 2.0_f64 * t1395 * t7961;
    let t91834 = 2.0_f64 * t1851 * t7240;
    (t91669, t91687, t91695, t91753, t91830, t91832, t91834)
}
