//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 650/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk650<F: Float>(t2127: F, t3679: F, t133: F, t3650: F, t793: F, t2139: F, t1138: F, t2123: F, t2138: F, t290: F, t2984: F, t3669: F, t791: F) -> (F, F, F, F, F) {
    let t3680 = t3679 * t2127;
    let t3685 = t3650 * t133;
    let t3686 = t3685 * t793;
    let t3689 = t3679 * t2139;
    let t3694 = F::cast_from(0.13170898365871023197e1_f64) * t2123 * t3680 + F::cast_from(0.13170898365871023197e1_f64) * t2984 * t1138 + F::cast_from(0.65854491829355115987e0_f64) * t791 * t3686 - F::cast_from(0.65854491829355115987e0_f64) * t2138 * t3689 + F::cast_from(0.65854491829355115987e0_f64) * t290 * t3669;
    (t3680, t3685, t3686, t3689, t3694)
}
