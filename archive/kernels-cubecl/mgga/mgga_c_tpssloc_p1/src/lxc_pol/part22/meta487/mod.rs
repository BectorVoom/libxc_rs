//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta487<F: Float>(t21238: F, t951: F, t10632: F, t21089: F, t13727: F, t5695: F, t1556: F, t5694: F, t913: F, t2842: F, t10756: F, t10771: F, t10811: F, t10828: F, t14263: F, t14271: F, t14337: F, t1569: F, t1581: F, t17355: F, t17428: F, t21115: F, t21195: F, t21198: F, t21207: F, t2930: F, t4411: F, t4449: F, t5759: F, t5762: F, t5775: F, t5791: F, t5794: F, t924: F, t943: F) -> (F, F, F, F, F, F, F, F) {
        let (t21239, t21242, t21247, t21251, t21252, t21253, t21255, t21256) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1907::<F>(t21238, t951, t10632, t21089, t13727, t5695, t1556, t5694, t913, t2842, t10756, t10771, t10811, t10828, t14263, t14271, t14337, t1569, t1581, t17355, t17428, t21115, t21195, t21198, t21207, t2930, t4411, t4449, t5759, t5762, t5775, t5791, t5794, t924, t943);
    (t21239, t21242, t21247, t21251, t21252, t21253, t21255, t21256)
}
