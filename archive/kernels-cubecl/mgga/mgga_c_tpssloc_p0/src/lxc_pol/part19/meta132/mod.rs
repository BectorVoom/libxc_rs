//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk695;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk696;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta132<F: Float>(t3590: F, t466: F, t1236: F, t225: F, t1239: F, t496: F, t68: F, t1251: F, t1243: F, t3534: F, t3032: F, t3502: F, t3499: F, t3507: F, t491: F, t1932: F, t3508: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3591, t3593, t3598, t3599, t3600, t3604) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk695::<F>(t3590, t466, t1236, t225, t1239, t496, t68, t1251, t1243, t3534);
        let (t3609, t3610) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk696::<F>(t3032, t3502, t3499);
        let (t3611, t3612) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk697::<F>(t3507, t491, t1932, t3508);
    (t3591, t3593, t3598, t3599, t3600, t3604, t3609, t3610, t3611, t3612)
}
