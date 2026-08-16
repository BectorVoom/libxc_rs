//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1188;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1189;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1190;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1191;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1192;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta264<F: Float>(t1235: F, t225: F, t497: F, t462: F, t457: F, t461: F, t491: F, t1240: F, t1251: F, t1190: F, t2144: F, t1193: F, t2127: F, t210: F, t2120: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7294, t7295) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1188::<F>(t1235, t225, t497);
        let (t7296, t7299) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1189::<F>(t462, t7295, t457, t461);
        let t7300 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1190::<F>(t491, t7299);
        let t7301 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1191::<F>(t1240, t225);
        let (t7302, t7303) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1192::<F>(t1251, t7301, t7300);
        let (t7306, t7309, t7310) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1193::<F>(t1190, t2144, t1193, t2127, t210, t2120);
    (t7294, t7295, t7296, t7299, t7300, t7301, t7302, t7303, t7306, t7309, t7310)
}
