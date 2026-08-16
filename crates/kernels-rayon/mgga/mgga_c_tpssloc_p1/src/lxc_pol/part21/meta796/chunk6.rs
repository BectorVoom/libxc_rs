//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2764/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764(t1519: f64, t4233: f64, t2631: f64, t40933: f64, t13263: f64, t13390: f64, t13397: f64, t13433: f64, t16758: f64, t16815: f64, t16828: f64, t17023: f64, t17030: f64, t2613: f64, t2633: f64, t2679: f64, t2684: f64, t4234: f64, t4281: f64, t4291: f64, t47386: f64, t5655: f64, t58166: f64, t808: f64, t812: f64, t829: f64, t9632: f64) -> (f64, f64, f64) {
    let t58226 = t1519 * t4233;
    let t58246 = t40933 * t2631;
    let t58261 = -36.0_f64 * t13263 * t13397 * t16815 - 4.0_f64 * t13433 * t4234 * t812 + 12.0_f64 * t16758 * t2633 * t4281 - 2.0_f64 * t16758 * t2679 * t4291 - t16815 * t2684 * t4291 + 6.0_f64 * t16815 * t4281 * t9632 + 24.0_f64 * t16815 * t47386 * t58246 - t17030 * t2679 * t4291 - 2.0_f64 * t4291 * t58166 * t829 - 4.0_f64 * t4291 * t58226 * t829 - 2.0_f64 * t13390 * t16828 + 2.0_f64 * t17023 * t808 + t2613 * t5655;
    (t58226, t58246, t58261)
}
