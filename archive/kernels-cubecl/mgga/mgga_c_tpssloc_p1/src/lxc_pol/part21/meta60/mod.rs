//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta60 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk434;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk435;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta60<F: Float>(t1178: F, t607: F, t1177: F, t1111: F, t1115: F, t457: F, t460: F, t974: F, t1173: F, t1174: F, t491: F, t1169: F, t221: F, t456: F, t1089: F, t1176: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1179, t1180, t1184) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk434::<F>(t1178, t607, t1177, t1111, t1115);
        let (t1186, t1187, t1190) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk435::<F>(t1184, t457, t460, t974, t1173, t1174, t1180);
        let (t1191, t1193, t1195, t1196) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk436::<F>(t1190, t491, t1169, t221, t456, t1089, t1176);
    (t1179, t1180, t1184, t1186, t1187, t1190, t1191, t1193, t1195, t1196)
}
