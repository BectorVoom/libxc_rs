//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 845/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk845<F: Float>(t626: F, t9095: F, t1045: F, t1055: F, t184: F, t188: F, t2671: F, t2679: F, t2703: F, t3461: F, t3467: F, t3488: F, t622: F, t634: F, t9020: F, t9034: F, t9037: F, t9043: F) -> (F, F) {
    let t9096 = t626 * t9095;
    let t9099 = F::cast_from(0.65854491829355115987e0_f64) * t9020 * t188 - F::cast_from(0.65854491829355115987e0_f64) * t3461 * t634 - F::cast_from(0.13170898365871023197e1_f64) * t2671 * t1055 + F::cast_from(0.26341796731742046394e1_f64) * t1045 * t2679 - F::cast_from(0.13170898365871023197e1_f64) * t1045 * t2703 + F::cast_from(0.13170898365871023197e1_f64) * t622 * t3467 - F::cast_from(0.39512695097613069591e1_f64) * t184 * t9034 + F::cast_from(0.26341796731742046394e1_f64) * t184 * t9037 - F::cast_from(0.65854491829355115987e0_f64) * t622 * t3488 + F::cast_from(0.13170898365871023197e1_f64) * t184 * t9043 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t9096;
    (t9096, t9099)
}
