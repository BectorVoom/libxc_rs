//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1285;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1286;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1287;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta227<F: Float>(t789: F, t9541: F, t2566: F, t786: F, t2578: F, t2570: F, t792: F, t154: F, t845: F, t205: F, t59: F, t8705: F, t207: F, t215: F, t782: F, t2690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9542, t9546, t9547, t9549, t9558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1285::<F>(t789, t9541, t2566, t786, t2578, t2570, t792, t154, t845);
        let (t9559, t9569) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1286::<F>(t205, t9558, t59, t8705);
        let (t9572, t9573) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1287::<F>(t207, t215, t9569, t2570, t782);
        let (t9576, t9577) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1288::<F>(t2690, t59, t154);
    (t9542, t9546, t9547, t9549, t9558, t9559, t9569, t9572, t9573, t9576, t9577)
}
