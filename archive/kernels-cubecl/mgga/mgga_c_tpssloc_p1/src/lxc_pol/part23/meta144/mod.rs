//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk684;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk685;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta144<F: Float>(t3131: F, t5872: F, t1021: F, t248: F, t360: F, t3151: F, t5392: F, t974: F, t5398: F, t998: F, t3146: F, t1044: F, t5681: F, t225: F, t5848: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5873 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk684::<F>(t3131, t5872);
        let (t5875, t5878) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk685::<F>(t1021, t248, t5873, t360, t5872);
        let (t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900, t5903) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk686::<F>(t1021, t248, t5878, t3151, t5392, t974, t5398, t998, t3146, t1044, t5681, t225, t5848);
    (t5873, t5875, t5878, t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900, t5903)
}
