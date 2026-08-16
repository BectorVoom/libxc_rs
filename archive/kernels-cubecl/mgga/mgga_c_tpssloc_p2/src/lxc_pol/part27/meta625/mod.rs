//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2109;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta625<F: Float>(t12813: F, t1873: F, t3941: F, t55341: F, t12524: F, t26542: F, t22479: F, t5371: F, t66940: F, t7769: F, t55353: F, t7015: F, t16524: F, t23896: F, t1458: F, t7010: F, t84004: F, t86582: F, t86606: F, t86610: F, t86612: F, t86614: F, t86616: F, t86619: F, t86622: F, t45560: F, t16521: F, t6534: F, t111: F, t7758: F, t55405: F, t23893: F, t26550: F, t112: F, t26509: F, t16535: F, t7467: F, t26135: F, t3938: F, t12816: F, t191: F, t192: F, t2020: F, t26161: F, t26162: F, t56404: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86625, t86629, t86631, t86633, t86635, t86637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2107::<F>(t12813, t1873, t3941, t55341, t12524, t26542, t22479, t5371, t66940, t7769, t55353, t7015);
        let t86640 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108::<F>(t16524, t23896, t12813, t1458, t7010, t84004, t86582, t86606, t86610, t86612, t86614, t86616, t86619, t86622, t86625, t86629, t86631, t86633, t86635, t86637);
        let (t86642, t86646, t86647, t86651, t86653, t86655) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2109::<F>(t45560, t7769, t16521, t6534, t111, t7758, t1873, t55405, t16524, t23893, t12524, t26550);
        let (t86656, t86660, t86668, t86673, t86676) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110::<F>(t112, t26509, t16535, t7467, t26135, t3938, t12816, t191, t192, t2020, t26161, t26162, t56404);
    (t86640, t86642, t86646, t86647, t86651, t86653, t86655, t86656, t86660, t86668, t86673, t86676)
}
