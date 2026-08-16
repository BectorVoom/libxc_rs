//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta114 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk773;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk774;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk775;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk776;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta114<F: Float>(t281: F, t2820: F, t415: F, t1114: F, t699: F, t1176: F, t241: F, t1097: F, t409: F, t422: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3293, t3294, t3295, t3297) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk773::<F>(t281, t2820, t415, t1114, t699, t1176, t241);
        let t3311 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk774::<F>(t1097);
        let t3312 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk775::<F>(t3311);
        let t3313 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk776::<F>(t3312, t409);
        let (t3314, t3315) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk777::<F>(t422);
    (t3293, t3294, t3295, t3297, t3311, t3312, t3313, t3314, t3315)
}
