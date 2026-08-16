//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3012/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012(t1057: f64, t61729: f64, t3199: f64, t61734: f64, t1061: f64, t11037: f64, t11051: f64, t11065: f64, t14574: f64, t14581: f64, t14590: f64, t14591: f64, t14608: f64, t14618: f64, t14623: f64, t14627: f64, t18131: f64, t18138: f64, t3040: f64, t3186: f64, t3202: f64, t43553: f64, t43554: f64, t4677: f64, t47857: f64, t5928: f64, t5932: f64, t5933: f64, t5936: f64) -> f64 {
    let t62994 = t61729 * t1057;
    let t63004 = t61734 * t3199;
    let t63022 = -36.0_f64 * t3040 * t43553 * t43554 * t5928 - 12.0_f64 * t11065 * t14590 * t5932 - 6.0_f64 * t11065 * t14590 * t5936 + 8.0_f64 * t18138 * t3186 * t4677 + 2.0_f64 * t1061 * t62994 - 4.0_f64 * t11037 * t18131 + 2.0_f64 * t11051 * t5933 - 4.0_f64 * t14574 * t14608 + 8.0_f64 * t14581 * t14618 - 12.0_f64 * t14591 * t47857 - 2.0_f64 * t14608 * t14623 - 2.0_f64 * t14608 * t14627 - t3202 * t63004;
    t63022
}
