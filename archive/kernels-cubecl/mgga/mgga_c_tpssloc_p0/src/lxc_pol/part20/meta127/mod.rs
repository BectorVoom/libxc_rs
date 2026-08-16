//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta127 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk833;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk834;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk835;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta127<F: Float>(t3076: F, t68: F, t369: F, t374: F, t376: F, t677: F, t370: F, t35: F, t365: F, t612: F, t364: F, t354: F, t1032: F, t1036: F, t1004: F, t1031: F, t1044: F, t248: F, t2776: F, t121: F, t1023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3077, t3078, t3082) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk832::<F>(t3076, t68, t369, t374, t376, t677);
        let (t3084, t3087) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk833::<F>(t3082, t370, t35, t365, t612);
        let (t3088, t3089, t3092, t3094, t3098) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk834::<F>(t3087, t364, t354, t1032, t1036, t1004, t1031, t1044, t248, t2776);
        let t3101 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk835::<F>(t121, t376);
        let t3103 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk836::<F>(t1023, t248, t3101);
    (t3077, t3078, t3082, t3084, t3087, t3088, t3089, t3092, t3094, t3098, t3101, t3103)
}
