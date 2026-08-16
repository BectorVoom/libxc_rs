//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1113/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1113(t1358: f64, t13734: f64, t689: f64, t1903: f64, t4131: f64, t4076: f64, t4077: f64, t9657: f64, t1444: f64, t5774: f64, t10171: f64, t13727: f64, t13733: f64, t1424: f64, t1904: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9650: f64) -> f64 {
    let t13735 = t13734 * t1358;
    let t13737 = 0.10975748638225852664e-1_f64 * t689 * t13735;
    let t13738 = t1903 * t4131;
    let t13739 = t4076 * t13738;
    let t13743 = t9657 * t1903 * t4077;
    let t13746 = t5774 * t1444;
    let t13747 = t4076 * t13746;
    let t13750 = 0.14634331517634470219e-1_f64 * t9632 - 0.54878743191129263322e-2_f64 * t9636 + t9639 - 0.13009920719177044025e-2_f64 * t9642 + t9650 - 0.65854491829355115987e0_f64 * t10171 * t1904 - 0.65049603595885220126e-3_f64 * t13727 - t13733 - t13737 + 0.13170898365871023197e1_f64 * t1424 * t13739 - 0.39512695097613069591e1_f64 * t1424 * t13743 + 0.26341796731742046394e1_f64 * t1424 * t13747;
    t13750
}
