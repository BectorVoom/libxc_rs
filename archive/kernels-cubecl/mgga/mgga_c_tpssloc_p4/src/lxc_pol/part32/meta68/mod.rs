//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta68 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk448;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk449;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk450;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk451;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk452;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk453;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta68<F: Float>(t1294: F, t763: F, t532: F, t571: F, t514: F, t25: F, t606: F, t517: F, zeta_threshold: F, t28: F, t1081: F, t215: F, t535: F, t782: F, t154: F, t547: F, t205: F, t210: F, t214: F, t792: F, t795: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1296, t1297) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk448::<F>(t1294, t763, t532, t571);
        let t1298 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk449::<F>(t514);
        let (t1301, t1302) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk450::<F>(t25, t1298, t606, t517, zeta_threshold);
        let t1307 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk451::<F>(t28, t1081, t1302, t1301, zeta_threshold);
        let (t1313, t1314) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk452::<F>(t215, t535, t782, t154, t547);
        let t1315 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk453::<F>(t1314, t205);
        let (t1317, t1322, t1323) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk454::<F>(t1307, t210, t214, t535, t792, t795, t1313, t1315);
    (t1296, t1297, t1298, t1302, t1307, t1313, t1314, t1315, t1317, t1322, t1323)
}
