//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk648;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk649;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk650;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta120<F: Float>(t422: F, t3265: F, t3313: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t1124: F, t1128: F, t1127: F, t432: F, t427: F, t1136: F, t1137: F, t3293: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3314, t3315) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk648::<F>(t422);
        let (t3316, t3318, t3324, t3327, t3330) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk649::<F>(t3265, t3315, t3313, t3236, t3238, t3245, t3250, t3254, t1124, t1128, t1127, t432);
        let (t3331, t3332, t3333) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk650::<F>(t3330, t427, t1136);
        let (t3334, t3351) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk651::<F>(t1137, t3333, t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
    (t3314, t3315, t3316, t3318, t3324, t3327, t3330, t3331, t3332, t3333, t3334, t3351)
}
