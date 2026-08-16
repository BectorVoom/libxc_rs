//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta474<F: Float>(t136: F, t3297: F, t78031: F, t78039: F, t1113: F, t78047: F, t78043: F, t1100: F, t78077: F, t3287: F, t78025: F, t11219: F, t78035: F, t1661: F, t71445: F, t71448: F, t18754: F, t5999: F, t18746: F, t43895: F, t63361: F, t78057: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t78084, t78087, t78090, t78093, t78095, t78097, t78100) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418::<F>(t136, t3297, t78031, t78039, t1113, t78047, t78043, t1100, t78077, t3287, t78025, t11219, t78035);
        let (t78103, t78105, t78107, t78109, t78112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419::<F>(t1661, t71445, t71448, t18754, t5999, t18746, t43895, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100);
    (t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109, t78112)
}
