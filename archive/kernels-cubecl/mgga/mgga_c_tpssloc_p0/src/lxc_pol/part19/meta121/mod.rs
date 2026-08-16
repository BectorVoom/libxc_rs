//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta121 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk652;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk653;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk654;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk655;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta121<F: Float>(t1137: F, t3351: F, t1127: F, t427: F, t435: F, t3333: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t449: F, t1143: F, t1147: F, t1146: F, t445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3352, t3355) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk652::<F>(t1137, t3351, t1127);
        let (t3356, t3357) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk653::<F>(t3355, t427);
        let (t3358, t3359) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk654::<F>(t435);
        let (t3360, t3368) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk655::<F>(t3333, t3359, t3236, t3238, t3245, t3250, t3254);
        let (t3369, t3371, t3374, t3375) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk656::<F>(t3368, t449, t1143, t1147, t1146, t445);
    (t3352, t3355, t3356, t3357, t3358, t3359, t3360, t3368, t3369, t3371, t3374, t3375)
}
