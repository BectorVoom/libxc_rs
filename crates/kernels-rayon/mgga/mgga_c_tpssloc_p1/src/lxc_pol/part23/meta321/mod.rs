//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1081;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta321(t22174: f64, t471: f64, t21762: f64, t248: f64, t3585: f64, t21510: f64, t4987: f64, t4582: f64, t1227: f64, t15503: f64, t15507: f64, t15569: f64, t15740: f64, t18357: f64, t18372: f64, t18376: f64, t18393: f64, t18972: f64, t18976: f64, t22154: f64, t22158: f64, t22162: f64, t22169: f64, t3577: f64, t488: f64, t5002: f64, t5005: f64, t5019: f64, t6192: f64, t6203: f64, t6221: f64, t6227: f64, t6232: f64, t11779: f64, t21758: f64, t1230: f64, t21776: f64, t21769: f64, t1156: f64, t21906: f64, t3400: f64, t1164: f64, t4869: f64, t6106: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22175, t22185, t22196, t22197, t22202) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1081(t22174, t471, t21762, t248, t3585, t21510, t4987, t4582, t1227, t15503, t15507, t15569, t15740, t18357, t18372, t18376, t18393, t18972, t18976, t22154, t22158, t22162, t22169, t3577, t488, t5002, t5005, t5019, t6192, t6203, t6221, t6227, t6232);
        let (t22208, t22214, t22218, t22222, t22224, t22226) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1082(t11779, t21758, t248, t1230, t21776, t21769, t1156, t21906, t3400, t1164, t4869, t6106);
    (t22175, t22185, t22196, t22197, t22202, t22208, t22214, t22218, t22222, t22224, t22226)
}
