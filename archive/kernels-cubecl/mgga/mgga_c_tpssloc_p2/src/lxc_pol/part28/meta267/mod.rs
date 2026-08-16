//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1137;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1138;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1139;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1140;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta267<F: Float>(t6976: F, t7736: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F, t2031: F, t7445: F, t5: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F, t112: F, t1774: F, t2039: F, t109: F, t7053: F, t7464: F, t510: F, t1458: F, t2075: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7737, t7738, t7740, t7741, t7742, t7782) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1137::<F>(t6976, t7736, t1992, t1834, t1998, t214, t1985, t2031, t7445);
        let (t7786, t7787, t7796) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1138::<F>(t5, t1860, t2032, t7026, t7034, t7428, t7432, t7435, t7782, t112, t1774, t2039);
        let t7801 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1139::<F>(t109, t7053, t7464);
        let t7802 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1140::<F>(t510, t7801);
        let t7806 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1141::<F>(t1458, t2075);
    (t7737, t7738, t7740, t7741, t7742, t7782, t7786, t7787, t7796, t7801, t7802, t7806)
}
