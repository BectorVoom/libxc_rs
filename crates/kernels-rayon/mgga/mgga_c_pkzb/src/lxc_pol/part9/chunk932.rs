//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 932/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk932(t626: f64, t7173: f64, t1045: f64, t1055: f64, t1784: f64, t1792: f64, t1813: f64, t184: f64, t188: f64, t2671: f64, t2679: f64, t2703: f64, t622: f64, t634: f64, t7097: f64, t7113: f64, t7117: f64, t7120: f64) -> (f64, f64) {
    let t7174 = t626 * t7173;
    let t7177 = 0.65854491829355115987e0_f64 * t7097 * t188 - 0.13170898365871023197e1_f64 * t2671 * t634 + 0.13170898365871023197e1_f64 * t1045 * t1792 - 0.65854491829355115987e0_f64 * t1045 * t1813 - 0.65854491829355115987e0_f64 * t1784 * t1055 + 0.26341796731742046394e1_f64 * t622 * t2679 - 0.13170898365871023197e1_f64 * t622 * t2703 - 0.39512695097613069591e1_f64 * t184 * t7113 + 0.26341796731742046394e1_f64 * t184 * t7117 + 0.13170898365871023197e1_f64 * t184 * t7120 - 0.65854491829355115987e0_f64 * t184 * t7174;
    (t7174, t7177)
}
