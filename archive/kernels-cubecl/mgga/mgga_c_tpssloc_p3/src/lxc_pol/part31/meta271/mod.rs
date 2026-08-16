//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta271 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1120;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1121;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1122;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1123;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1124;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta271<F: Float>(t1774: F, t2039: F, t109: F, t7053: F, t7464: F, t510: F, t1458: F, t2075: F, t2057: F, t7475: F, t1492: F, t2047: F, t7074: F, t7076: F, t7078: F, t7082: F, t7494: F, t7498: F, t7501: F, t7504: F, t7506: F, t7508: F, t218: F, t1527: F, t2053: F, t2718: F, t1510: F, t7101: F, t235: F, t1499: F, t2051: F, t226: F, t7095: F, t7097: F, t7522: F, t7526: F, t7530: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t7796 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1120::<F>(t1774, t2039);
        let t7801 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1121::<F>(t109, t7053, t7464);
        let t7802 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1122::<F>(t510, t7801);
        let (t7806, t7809, t7815, t7823) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1123::<F>(t1458, t2075, t2057, t7475, t1492, t2047, t7074, t7076, t7078, t7082, t7494, t7498, t7501, t7504, t7506, t7508);
        let (t7824, t7830) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1124::<F>(t218, t7823, t1527, t2053, t2718);
        let (t7837, t7839, t7841) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1125::<F>(t1510, t7101, t235, t7823, t1499, t2051, t226, t7095, t7097, t7522, t7526, t7530, t812);
    (t7796, t7801, t7802, t7806, t7809, t7815, t7823, t7824, t7830, t7837, t7839, t7841)
}
