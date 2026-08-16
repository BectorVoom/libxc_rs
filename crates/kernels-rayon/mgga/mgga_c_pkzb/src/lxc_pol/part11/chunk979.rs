//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 979/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk979(t10727: f64, t626: f64, t1045: f64, t1055: f64, t10676: f64, t10686: f64, t10689: f64, t184: f64, t188: f64, t3461: f64, t3467: f64, t3488: f64) -> (f64, f64) {
    let t10728 = t626 * t10727;
    let t10731 = 0.65854491829355115987e0_f64 * t10676 * t188 - 0.19756347548806534796e1_f64 * t3461 * t1055 + 0.39512695097613069591e1_f64 * t1045 * t3467 - 0.19756347548806534796e1_f64 * t1045 * t3488 - 0.39512695097613069591e1_f64 * t184 * t10686 + 0.39512695097613069591e1_f64 * t184 * t10689 - 0.65854491829355115987e0_f64 * t184 * t10728;
    (t10728, t10731)
}
