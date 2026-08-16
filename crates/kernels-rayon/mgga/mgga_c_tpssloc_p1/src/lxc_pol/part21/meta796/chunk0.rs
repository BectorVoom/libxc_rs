//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2758/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758(t40817: f64, t13191: f64, t13487: f64, t16592: f64, t16606: f64, t17120: f64, t1877: f64, t193: f64, t2378: f64, t2522: f64, t2553: f64, t2749: f64, t39549: f64, t39563: f64, t40772: f64, t4307: f64, t4310: f64, t4314: f64, t5664: f64, t58071: f64, t58080: f64, t58085: f64, t58090: f64) -> (f64, f64) {
    let t58094 = 0.17315859105681463759e2_f64 * t40817;
    let t58095 = -6.0_f64 * t1877 * t2749 * t40772 * t5664 + 24.0_f64 * t13191 * t4310 * t4314 + 12.0_f64 * t13487 * t17120 * t2522 + 6.0_f64 * t16592 * t2553 * t4314 + 3.0_f64 * t16606 * t2522 * t2553 + 12.0_f64 * t193 * t2378 * t58090 - 12.0_f64 * t2522 * t4307 * t58071 + t39549 + t39563 + t58080 + t58085 - t58094;
    (t58094, t58095)
}
