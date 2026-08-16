//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1040/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1040(t1862: f64, t8308: f64, t131: f64, t63: f64, t31688: f64, t31693: f64, t2303: f64, t31691: f64, t8513: f64, t31687: f64, t8515: f64, t9231: f64) -> (f64, f64, f64, f64, f64) {
    let t115833 = t8308 * t1862;
    let t115834 = t63 * t131 * t115833;
    let t115837 = t31688 * t31693;
    let t115842 = t8513 * t31691 * t2303;
    let t115846 = t9231 * t31687 * t8515;
    (t115833, t115834, t115837, t115842, t115846)
}
