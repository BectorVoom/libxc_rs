//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2766/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766(t13396: f64, t1499: f64, t13380: f64, t13398: f64, t13414: f64, t13423: f64, t13448: f64, t16673: f64, t16679: f64, t16935: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t40895: f64, t4166: f64, t4182: f64, t4234: f64, t4281: f64, t4291: f64, t5585: f64, t5645: f64, t58204: f64, t812: f64, t9612: f64) -> f64 {
    let t58313 = t1499 * t13396;
    let t58337 = 8.0_f64 * t13380 * t16935 * t4281 - 4.0_f64 * t13380 * t4234 * t4291 + 2.0_f64 * t40895 * t5585 * t812 + 8.0_f64 * t4182 * t4281 * t58204 - 12.0_f64 * t13398 * t58313 - 2.0_f64 * t13414 * t4166 - 2.0_f64 * t13423 * t4166 + 2.0_f64 * t13448 * t1499 + 2.0_f64 * t16673 * t2729 - 2.0_f64 * t16673 * t2733 - t16673 * t2736 - 4.0_f64 * t16679 * t2617 + 2.0_f64 * t5645 * t9612;
    t58337
}
