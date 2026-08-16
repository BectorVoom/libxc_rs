//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2391/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2391(t12267: f64, t3789: f64, t3798: f64, t3802: f64, t3734: f64, t3792: f64, t3719: f64, t1314: f64, t9569: f64, t1329: f64, t12189: f64, t3770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39952 = t12267 * t3789;
    let t39955 = t12267 * t3798;
    let t39975 = t12267 * t3802;
    let t39978 = t3792 * t3734;
    let t39993 = t3792 * t3719;
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    let t40008 = t12189 * t3770;
    (t39952, t39955, t39975, t39978, t39993, t40005, t40006, t40008)
}
