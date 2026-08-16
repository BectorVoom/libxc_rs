//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1515;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1516;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta265<F: Float>(t2578: F, t9546: F, t2570: F, t792: F, t118: F, t2379: F, t794: F, t2553: F, t2576: F, t154: F, t845: F, t205: F, t59: F, t8705: F, t207: F, t215: F, t782: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9547, t9549, t9551, t9552, t9555, t9556, t9558, t9559) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1515::<F>(t2578, t9546, t2570, t792, t118, t2379, t794, t2553, t2576, t154, t845, t205);
        let t9569 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1516::<F>(t59, t8705);
        let (t9572, t9573) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1517::<F>(t207, t215, t9569, t2570, t782);
    (t9547, t9549, t9551, t9552, t9555, t9556, t9558, t9559, t9569, t9572, t9573)
}
