//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2308;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta581<F: Float>(t12050: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15898: F, t15911: F, t15916: F, t15917: F, t15923: F, t19599: F, t9780: F, t9789: F, t172: F, t6320: F, t763: F, t15972: F, t12097: F, t12106: F, t12111: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t15976: F, t9793: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19677, t19678, t19679) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2308::<F>(t12050, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15898, t15911, t15916, t15917, t15923, t19599, t9780, t9789);
        let (t19681, t19683, t19684, t19685, t19686, t19687, t19688) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2309::<F>(t172, t6320, t763, t15972, t12097, t12106, t12111, t12103, t12105, t12109, t12114, t12116, t12118, t15976, t9793, t9797, t9820, t9824);
    (t19677, t19678, t19679, t19681, t19683, t19684, t19685, t19686, t19687, t19688)
}
