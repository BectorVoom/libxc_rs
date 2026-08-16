//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta438<F: Float>(t1216: F, t4733: F, t3578: F, t1653: F, t3494: F, t1090: F, t5012: F, t3490: F, t4993: F, t248: F, t3521: F, t1227: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15469, t15470, t15473, t15474, t15477, t15478, t15484, t15486, t15488) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1978::<F>(t1216, t4733, t3578, t1653, t3494, t1090, t5012, t3490, t4993, t248, t3521, t1227);
    (t15469, t15470, t15473, t15474, t15477, t15478, t15484, t15486, t15488)
}
