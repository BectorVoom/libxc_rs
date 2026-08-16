//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1782;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta529<F: Float>(t22695: F, t22704: F, t22705: F, t22863: F, t6979: F, t22641: F, t3749: F, t6978: F, t80854: F, t22719: F, t6897: F, t794: F, t1984: F, t80845: F, t2010: F, t6973: F, t80742: F, t22724: F, t22727: F, t22894: F, t80670: F, t22882: F, t22892: F, t22893: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81050, t81061, t81064, t81066, t81069) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1782::<F>(t22695, t22704, t22705, t22863, t6979, t22641, t3749, t6978, t80854, t22719, t6897, t794);
        let (t81071, t81072, t81074, t81076, t81080, t81083) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1783::<F>(t1984, t80845, t2010, t6973, t80742, t22724, t22727, t22894, t80670, t22882, t22892, t22893);
    (t81050, t81061, t81064, t81066, t81069, t81071, t81072, t81074, t81076, t81080, t81083)
}
