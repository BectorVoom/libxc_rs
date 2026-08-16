//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1883;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1884;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1885;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta402<F: Float>(t14488: F, t360: F, t1021: F, t248: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t1618: F, t3043: F, t3057: F, t3064: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4652: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14489, t14491, t14495, t14501, t14503, t14506, t14507) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1883::<F>(t14488, t360, t1021, t248, t3053, t4644, t10422, t4578, t3070, t1603, t3030, t3032);
        let t14508 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1884::<F>(t14507, t3129);
        let t14511 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1885::<F>(t14507, t3038);
        let t14523 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1886::<F>(t1020, t10937, t10962, t10982, t10985, t10994, t11003, t14235, t14491, t14495, t14503, t14508, t14511, t1618, t3043, t3057, t3064, t3070, t3114, t3123, t3134, t4579, t4641, t4644, t4652);
    (t14489, t14491, t14495, t14501, t14503, t14506, t14507, t14508, t14511, t14523)
}
