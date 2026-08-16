//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2109;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta625(t12813: f64, t1873: f64, t3941: f64, t55341: f64, t12524: f64, t26542: f64, t22479: f64, t5371: f64, t66940: f64, t7769: f64, t55353: f64, t7015: f64, t16524: f64, t23896: f64, t1458: f64, t7010: f64, t84004: f64, t86582: f64, t86606: f64, t86610: f64, t86612: f64, t86614: f64, t86616: f64, t86619: f64, t86622: f64, t45560: f64, t16521: f64, t6534: f64, t111: f64, t7758: f64, t55405: f64, t23893: f64, t26550: f64, t112: f64, t26509: f64, t16535: f64, t7467: f64, t26135: f64, t3938: f64, t12816: f64, t191: f64, t192: f64, t2020: f64, t26161: f64, t26162: f64, t56404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86625, t86629, t86631, t86633, t86635, t86637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2107(t12813, t1873, t3941, t55341, t12524, t26542, t22479, t5371, t66940, t7769, t55353, t7015);
        let t86640 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108(t16524, t23896, t12813, t1458, t7010, t84004, t86582, t86606, t86610, t86612, t86614, t86616, t86619, t86622, t86625, t86629, t86631, t86633, t86635, t86637);
        let (t86642, t86646, t86647, t86651, t86653, t86655) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2109(t45560, t7769, t16521, t6534, t111, t7758, t1873, t55405, t16524, t23893, t12524, t26550);
        let (t86656, t86660, t86668, t86673, t86676) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110(t112, t26509, t16535, t7467, t26135, t3938, t12816, t191, t192, t2020, t26161, t26162, t56404);
    (t86640, t86642, t86646, t86647, t86651, t86653, t86655, t86656, t86660, t86668, t86673, t86676)
}
