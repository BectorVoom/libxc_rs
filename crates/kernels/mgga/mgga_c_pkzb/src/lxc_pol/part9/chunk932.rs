//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 932/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk932<F: Float>(t626: F, t7173: F, t1045: F, t1055: F, t1784: F, t1792: F, t1813: F, t184: F, t188: F, t2671: F, t2679: F, t2703: F, t622: F, t634: F, t7097: F, t7113: F, t7117: F, t7120: F) -> (F, F) {
    let t7174 = t626 * t7173;
    let t7177 = F::new(0.65854491829355115987e0) * t7097 * t188 - F::new(0.13170898365871023197e1) * t2671 * t634 + F::new(0.13170898365871023197e1) * t1045 * t1792 - F::new(0.65854491829355115987e0) * t1045 * t1813 - F::new(0.65854491829355115987e0) * t1784 * t1055 + F::new(0.26341796731742046394e1) * t622 * t2679 - F::new(0.13170898365871023197e1) * t622 * t2703 - F::new(0.39512695097613069591e1) * t184 * t7113 + F::new(0.26341796731742046394e1) * t184 * t7117 + F::new(0.13170898365871023197e1) * t184 * t7120 - F::new(0.65854491829355115987e0) * t184 * t7174;
    (t7174, t7177)
}
