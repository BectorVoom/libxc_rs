//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta139 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk901;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk902;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk903;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk904;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta139<F: Float>(t1137: F, t3333: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F, t1127: F, t427: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3334, t3339, t3346, t3351) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk901::<F>(t1137, t3333, t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
        let (t3352, t3355) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk902::<F>(t1137, t3351, t1127);
        let t3356 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk903::<F>(t3355);
        let t3357 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk904::<F>(t3356, t427);
        let (t3358, t3359) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk905::<F>(t435);
    (t3334, t3339, t3346, t3351, t3352, t3355, t3356, t3357, t3358, t3359)
}
