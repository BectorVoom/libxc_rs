//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta67 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk441;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk442;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk443;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk444;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk445;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta67<F: Float>(t1314: F, t205: F, t1307: F, t210: F, t214: F, t535: F, t792: F, t795: F, t1313: F, t562: F, t541: F, t801: F, t119: F, t225: F, t554: F, t544: F, t68: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1315 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk441::<F>(t1314, t205);
        let (t1317, t1322, t1323) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk442::<F>(t1307, t210, t214, t535, t792, t795, t1313, t1315);
        let (t1324, t1327, t1329) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk443::<F>(t1323, t562, t541, t801, t119, t1307, t210);
        let t1332 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk444::<F>(t1323, t225);
        let (t1333, t1336) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk445::<F>(t1332, t554, t544, t68);
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk446::<F>(t551);
    (t1315, t1317, t1322, t1323, t1324, t1327, t1329, t1332, t1333, t1336, t1337, t1338)
}
