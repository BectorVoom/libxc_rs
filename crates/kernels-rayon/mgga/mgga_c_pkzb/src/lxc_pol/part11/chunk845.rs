//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 845/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk845(t626: f64, t9095: f64, t1045: f64, t1055: f64, t184: f64, t188: f64, t2671: f64, t2679: f64, t2703: f64, t3461: f64, t3467: f64, t3488: f64, t622: f64, t634: f64, t9020: f64, t9034: f64, t9037: f64, t9043: f64) -> (f64, f64) {
    let t9096 = t626 * t9095;
    let t9099 = 0.65854491829355115987e0_f64 * t9020 * t188 - 0.65854491829355115987e0_f64 * t3461 * t634 - 0.13170898365871023197e1_f64 * t2671 * t1055 + 0.26341796731742046394e1_f64 * t1045 * t2679 - 0.13170898365871023197e1_f64 * t1045 * t2703 + 0.13170898365871023197e1_f64 * t622 * t3467 - 0.39512695097613069591e1_f64 * t184 * t9034 + 0.26341796731742046394e1_f64 * t184 * t9037 - 0.65854491829355115987e0_f64 * t622 * t3488 + 0.13170898365871023197e1_f64 * t184 * t9043 - 0.65854491829355115987e0_f64 * t184 * t9096;
    (t9096, t9099)
}
