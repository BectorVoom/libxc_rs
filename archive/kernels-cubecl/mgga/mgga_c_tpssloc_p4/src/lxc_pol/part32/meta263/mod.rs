//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta263 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1181;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1182;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1183;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1184;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1185;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1186;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta263<F: Float>(t25: F, t265: F, t394: F, t2165: F, t671: F, t6834: F, t2116: F, t40: F, t607: F, t6678: F, t1170: F, t2123: F, t2121: F, t2127: F, t6686: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1176: F, t461: F, t491: F, t225: F, t497: F, t1090: F, t1186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7271, t7274, t7279, t7280, t7282, t7283) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1181::<F>(t25, t265, t394, t2165, t671, t6834, t2116, t40, t607, t6678, t1170, t2123, t2121, t2127, t6686, dens_threshold, rho0, zeta_threshold);
        let t7284 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1182::<F>(t1176, t461);
        let t7285 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1183::<F>(t491, t7284);
        let t7286 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1184::<F>(t225, t497);
        let t7287 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1185::<F>(t1090, t7286);
        let t7288 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1186::<F>(t7285, t7287);
        let t7291 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1187::<F>(t1186, t2123);
    (t7271, t7274, t7279, t7280, t7282, t7283, t7284, t7285, t7286, t7287, t7288, t7291)
}
