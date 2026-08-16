//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1783/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1783(t13381: f64, t13385: f64, t13388: f64, t13390: f64, t13393: f64, t13397: f64, t13398: f64, t13401: f64, t13404: f64, t13407: f64, t13414: f64, t13417: f64, t13423: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t4166: f64, t4281: f64, t4291: f64, t4292: f64, t4296: f64, t812: f64) -> f64 {
    let t13425 = -2.0_f64 * t13381 * t4291 + 4.0_f64 * t13385 * t4281 - t13388 * t4291 - 2.0_f64 * t13390 * t4292 + 4.0_f64 * t13393 * t4281 - 6.0_f64 * t13397 * t13398 + 6.0_f64 * t13401 * t4281 + 2.0_f64 * t13404 * t4281 - 2.0_f64 * t13407 * t812 - t13414 * t812 + 2.0_f64 * t13417 * t812 - t13423 * t812 - 2.0_f64 * t2617 * t4296 + 2.0_f64 * t2729 * t4166 - 2.0_f64 * t2733 * t4166 - t2736 * t4166;
    t13425
}
