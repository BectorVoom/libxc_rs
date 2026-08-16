//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1156;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1157;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1158;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1159;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1160;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta183<F: Float>(t4395: F, t913: F, t893: F, t1556: F, t2844: F, t912: F, t2842: F, t2766: F, t2848: F, t4335: F, t4340: F, t4345: F, t4349: F, t1561: F, t923: F, t1569: F, t931: F, t2824: F, t2868: F, t2875: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F, t932: F, t1568: F, t2888: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4396 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1156::<F>(t4395, t913);
        let (t4398, t4399) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1157::<F>(t4396, t893, t1556, t2844);
        let (t4400, t4402, t4408, t4411) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1158::<F>(t4399, t912, t2842, t2766, t2848, t4335, t4340, t4345, t4349, t1561, t923);
        let (t4416, t4433) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1159::<F>(t1569, t931, t2766, t2824, t2868, t2875, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387, t4390, t4393);
        let t4434 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1160::<F>(t4433, t932);
        let t4437 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1161::<F>(t1568, t2888);
    (t4396, t4398, t4399, t4400, t4402, t4408, t4411, t4416, t4433, t4434, t4437)
}
