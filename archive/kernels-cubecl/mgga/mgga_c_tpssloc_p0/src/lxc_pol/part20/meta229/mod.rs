//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1313;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1314;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1315;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta229<F: Float>(t2519: F, t751: F, t2393: F, t763: F, t2374: F, t2749: F, t2752: F, t702: F, t9454: F, t2411: F, t2409: F, t681: F, t125: F, t141: F, t2413: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9462, t9463, t9467) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1313::<F>(t2519, t751, t2393, t763);
        let (t9469, t9470, t9474, t9476) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1314::<F>(t2374, t9467, t2749, t2752, t702, t9454, t2411);
        let (t9478, t9479, t9481, t9482, t9484) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1315::<F>(t2409, t681, t125, t141, t2413, t9454);
    (t9462, t9463, t9467, t9469, t9470, t9474, t9476, t9478, t9479, t9481, t9482, t9484)
}
