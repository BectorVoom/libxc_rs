//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1623;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1624;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1625;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta298<F: Float>(t2775: F, t283: F, t61: F, t135: F, t3142: F, t973: F, t3147: F, t3152: F, t248: F, t3101: F, t3132: F, t3130: F, t225: F, t3167: F, t10947: F, t3185: F, t3199: F, t1014: F, t10471: F, t10470: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10969, t10970, t10982, t10985, t10994, t11002, t11003) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1623::<F>(t2775, t283, t61, t135, t3142, t973, t3147, t3152, t248, t3101, t3132, t3130);
        let (t11010, t11034) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1624::<F>(t225, t3167, t10947, t3185);
        let t11037 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1625::<F>(t10947, t3199);
        let (t11045, t11046) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1626::<F>(t1014, t10471, t10470);
    (t10969, t10970, t10982, t10985, t10994, t11002, t11003, t11010, t11034, t11037, t11045, t11046)
}
