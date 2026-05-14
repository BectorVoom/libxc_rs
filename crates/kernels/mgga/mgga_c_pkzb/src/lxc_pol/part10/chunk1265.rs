//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1265/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1265<F: Float>(t1045: F, t1055: F, t17121: F, t1784: F, t1791: F, t1792: F, t1813: F, t184: F, t24801: F, t24834: F, t24869: F, t24898: F, t2671: F, t2679: F, t2703: F, t3461: F, t3466: F, t3488: F, t622: F, t626: F, t7097: F, t7113: F, t7117: F, t7120: F, t9034: F, t9043: F) -> (F,) {
    let t24922 = 0.15805078039045227836e2 * t184 * t17121 * t3466 * t1791 - 0.65854491829355115987e0 * t3461 * t1813 + 0.13170898365871023197e1 * t3461 * t1792 - 0.65854491829355115987e0 * t184 * t626 * (t24801 + t24834 + t24869 + t24898) + 0.52683593463484092788e1 * t1045 * t7117 + 0.52683593463484092788e1 * t2671 * t2679 - 0.65854491829355115987e0 * t1784 * t3488 - 0.79025390195226139182e1 * t622 * t9034 - 0.26341796731742046394e1 * t2671 * t2703 + 0.26341796731742046394e1 * t622 * t9043 - 0.79025390195226139182e1 * t1045 * t7113 + 0.26341796731742046394e1 * t1045 * t7120 - 0.13170898365871023197e1 * t7097 * t1055;
    (t24922,)
}
