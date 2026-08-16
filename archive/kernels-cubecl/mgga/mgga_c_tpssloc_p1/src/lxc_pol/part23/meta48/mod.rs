//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta48 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk310;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk311;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk312;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk313;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta48<F: Float>(t1147: F, t440: F, t1086: F, t1111: F, t448: F, t300: F, t134: F, t457: F, t461: F, t221: F, t456: F, t51: F, t972: F, t404: F, t405: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1148, t1150, t1153, t1156) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk310::<F>(t1147, t440, t1086, t1111, t448);
        let t1164 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk311::<F>(t300, t440);
        let (t1169, t1171, t1173, t1174) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk312::<F>(t134, t457, t461, t221, t456, t51, t972);
        let t1176 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk313::<F>(t404, t405);
        let t1177 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk314::<F>(t1176, t974);
    (t1148, t1150, t1153, t1156, t1164, t1169, t1171, t1173, t1174, t1176, t1177)
}
