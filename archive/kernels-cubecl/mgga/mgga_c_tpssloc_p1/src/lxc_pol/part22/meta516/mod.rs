//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1980;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta516<F: Float>(t22174: F, t471: F, t21762: F, t248: F, t3585: F, t21510: F, t4987: F, t4582: F, t1227: F, t15503: F, t15507: F, t15569: F, t15740: F, t18357: F, t18372: F, t18376: F, t18393: F, t18972: F, t18976: F, t22154: F, t22158: F, t22162: F, t22169: F, t3577: F, t488: F, t5002: F, t5005: F, t5019: F, t6192: F, t6203: F, t6221: F, t6227: F, t6232: F, t11779: F, t21758: F, t1230: F, t21776: F, t21769: F, t1156: F, t21906: F, t3400: F, t1164: F, t4869: F, t6106: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22175, t22185, t22196, t22197, t22202) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1980::<F>(t22174, t471, t21762, t248, t3585, t21510, t4987, t4582, t1227, t15503, t15507, t15569, t15740, t18357, t18372, t18376, t18393, t18972, t18976, t22154, t22158, t22162, t22169, t3577, t488, t5002, t5005, t5019, t6192, t6203, t6221, t6227, t6232);
        let (t22208, t22214, t22218, t22222, t22224, t22226) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1981::<F>(t11779, t21758, t248, t1230, t21776, t21769, t1156, t21906, t3400, t1164, t4869, t6106);
    (t22175, t22185, t22196, t22197, t22202, t22208, t22214, t22218, t22222, t22224, t22226)
}
