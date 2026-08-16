//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1232;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1233;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1234;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1235;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1236;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1237;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta263<F: Float>(t466: F, t7348: F, t2145: F, t225: F, t1251: F, t2154: F, t3598: F, t1170: F, t2148: F, t2121: F, t7284: F, t477: F, t491: F, t1090: F, t1186: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7349, t7351) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1232::<F>(t466, t7348, t2145, t225);
        let t7356 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1233::<F>(t1251, t2154, t3598);
        let (t7359, t7361, t7362) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1234::<F>(t1170, t2148, t2121, t225, t7284);
        let t7363 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1235::<F>(t477, t491);
        let t7364 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1236::<F>(t1090, t7363);
        let t7365 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1237::<F>(t7362, t7364);
        let t7368 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1238::<F>(t1186, t2148);
    (t7349, t7351, t7356, t7359, t7361, t7362, t7363, t7364, t7365, t7368)
}
