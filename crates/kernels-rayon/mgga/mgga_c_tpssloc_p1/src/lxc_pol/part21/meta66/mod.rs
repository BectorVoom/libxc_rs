//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk478;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk479;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk480;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk481;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta66(t1314: f64, t205: f64, t1307: f64, t210: f64, t214: f64, t535: f64, t792: f64, t795: f64, t1313: f64, t562: f64, t541: f64, t801: f64, t119: f64, t225: f64, t554: f64, t544: f64, t68: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1315 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk478(t1314, t205);
        let (t1317, t1322, t1323) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk479(t1307, t210, t214, t535, t792, t795, t1313, t1315);
        let (t1324, t1327, t1329, t1332) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk480(t1323, t562, t541, t801, t119, t1307, t210, t225);
        let (t1333, t1336) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk481(t1332, t554, t544, t68);
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk482(t551);
    (t1315, t1317, t1322, t1323, t1324, t1327, t1329, t1332, t1333, t1336, t1337, t1338)
}
