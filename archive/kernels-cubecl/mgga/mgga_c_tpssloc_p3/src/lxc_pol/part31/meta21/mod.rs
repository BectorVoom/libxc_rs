//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta21 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk149;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk150;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk151;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk152;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk153;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk154;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta21<F: Float>(t120: F, t61: F, t283: F, t374: F, t339: F, t350: F, t370: F, t349: F, t362: F, t68: F, t353: F, t254: F, t193: F, t293: F, t328: F, t330: F, t336: F, t265: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t375, t376) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk149::<F>(t120, t61, t283);
        let t378 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk150::<F>(t374, t375, t376);
        let t381 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk151::<F>(t339, t350, t370, t378);
        let (t382, t383) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk152::<F>(t349, t381, t362, t68);
        let t384 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk153::<F>(t381, t383);
        let (t386, t388) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk154::<F>(t353, t384, t254);
        let (t390, t396, t394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk155::<F>(t382, t388, t193, t293, t328, t330, t336, t265);
    (t375, t376, t378, t381, t382, t383, t384, t386, t388, t390, t396, t394)
}
