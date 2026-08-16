//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta110 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk746;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk747;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk748;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk749;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta110<F: Float>(t3082: F, t370: F, t1032: F, t1036: F, t121: F, t376: F, t1023: F, t248: F, t1020: F, t1017: F, t1030: F, t1015: F, t1012: F, t1009: F, t990: F, t1011: F, t1019: F, t1004: F, t1040: F, t1013: F, t361: F, t363: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3084, t3092, t3101) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk746::<F>(t3082, t370, t1032, t1036, t121, t376);
        let (t3103, t3104, t3108, t3109) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk747::<F>(t1023, t248, t3101, t1020, t1017, t1030, t1015, t1012);
        let (t3112, t3114) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk748::<F>(t1009, t990, t1011, t1019);
        let t3117 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk749::<F>(t1004, t1040);
        let (t3127, t3128) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk750::<F>(t1013, t361, t363);
    (t3084, t3092, t3101, t3103, t3104, t3108, t3109, t3112, t3114, t3117, t3127, t3128)
}
