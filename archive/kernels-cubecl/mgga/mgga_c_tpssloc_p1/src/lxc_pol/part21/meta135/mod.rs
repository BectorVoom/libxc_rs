//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta135 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk903;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk904;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk905;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk906;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta135<F: Float>(t1117: F, t1118: F, t3264: F, t407: F, t410: F, t1102: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t1100: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3265 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk903::<F>(t1117);
        let (t3266, t3268, t3270) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk904::<F>(t1118, t3265, t3264, t407, t410);
        let t3271 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk905::<F>(t1102);
        let (t3272, t3274, t3279) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk906::<F>(t3270, t3271, t3236, t3238, t3245, t3250, t3254);
        let (t3280, t3282, t3287) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk907::<F>(t1100, t3279, t3236, t407);
    (t3265, t3266, t3268, t3270, t3271, t3272, t3274, t3279, t3280, t3282, t3287)
}
