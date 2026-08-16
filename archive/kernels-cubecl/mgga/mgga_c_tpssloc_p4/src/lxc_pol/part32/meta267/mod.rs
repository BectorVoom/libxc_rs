//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta267 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1210;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1211;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1212;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1213;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1214;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1215;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta267<F: Float>(t1170: F, t2148: F, t2121: F, t225: F, t7284: F, t477: F, t491: F, t1090: F, t1186: F, t50: F, t6794: F, t131: F, t467: F, t1009: F, t461: F, t1209: F, t475: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7359, t7361, t7362) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1210::<F>(t1170, t2148, t2121, t225, t7284);
        let t7363 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1211::<F>(t477, t491);
        let (t7364, t7365) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1212::<F>(t1090, t7363, t7362);
        let t7368 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1213::<F>(t1186, t2148);
        let (t7371, t7372, t7373) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1214::<F>(t50, t6794, t131, t467);
        let t7375 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1215::<F>(t1009, t461, t1209);
        let t7376 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1216::<F>(t475, t68);
    (t7359, t7361, t7362, t7363, t7364, t7365, t7368, t7371, t7372, t7373, t7375, t7376)
}
