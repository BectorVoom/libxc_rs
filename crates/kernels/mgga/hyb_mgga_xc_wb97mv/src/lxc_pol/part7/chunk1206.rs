//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1206/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1206<F: Float>(t24785: F, t24802: F, t24824: F, t24832: F, t24834: F, t24836: F, t24838: F, t24840: F, t24844: F, t24846: F, t24849: F, t24851: F, t24854: F, t24856: F, t24858: F, t24861: F, t24876: F, t24878: F) -> (F,) {
    let t29051 = -t24785 / 32.0 - t24802 / 32.0 - t24824 / 48.0 - 5.0 / 144.0 * t24832 - t24834 / 8.0 - t24836 / 8.0 + t24838 / 24.0 + t24840 / 24.0 - 41.0 / 48.0 * t24844 - 5.0 / 144.0 * t24846 + t24849 / 24.0 - t24851 / 16.0 + t24854 / 24.0 - t24856 / 32.0 - t24858 / 16.0 - t24861 / 16.0 - t24876 / 32.0 - t24878 / 16.0;
    (t29051,)
}
