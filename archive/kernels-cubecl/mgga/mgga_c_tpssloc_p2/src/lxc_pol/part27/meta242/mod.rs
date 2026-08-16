//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta242 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1159;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1160;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1161;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1162;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1163;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1164;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta242<F: Float>(t815: F, t829: F, t6605: F, t1898: F, t808: F, t249: F, t59: F, t814: F, t240: F, t812: F, t831: F, t1899: F, t838: F, t234: F, t849: F, t6580: F, t6582: F, t6587: F, t6594: F, t6603: F, t218: F, t1903: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6606, t6607, t6609, t6610, t6612) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1159::<F>(t815, t829, t6605, t1898, t808, t249, t59, t814);
        let t6613 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1160::<F>(t240, t6612);
        let t6614 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1161::<F>(t6613, t812);
        let (t6615, t6618, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1162::<F>(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1163::<F>(t6620, t812);
        let t6624 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1164::<F>(t6621, t849, t6580, t6582, t6587, t6594, t6603, t6607, t6610, t6615, t6618);
        let (t6625, t6627) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1165::<F>(t218, t6624, t1903, t225);
    (t6606, t6609, t6612, t6613, t6614, t6618, t6619, t6620, t6621, t6624, t6625, t6627)
}
