//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk308;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta47<F: Float>(t1086: F, t432: F, t427: F, t1111: F, t435: F, t445: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1122, t1127, t1128, t1129, t1131, t1134, t1137) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk308::<F>(t1086, t432, t427, t1111, t435);
        let (t1141, t1146, t1147) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk309::<F>(t1086, t445);
    (t1122, t1127, t1128, t1129, t1131, t1134, t1137, t1141, t1146, t1147)
}
