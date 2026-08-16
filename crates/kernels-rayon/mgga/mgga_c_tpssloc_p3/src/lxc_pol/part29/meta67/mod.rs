//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk452;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk453;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk454;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk455;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk456;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk457;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta67(t1294: f64, t763: f64, t532: f64, t571: f64, t514: f64, t25: f64, t606: f64, t517: f64, zeta_threshold: f64, t28: f64, t1081: f64, t215: f64, t535: f64, t782: f64, t154: f64, t547: f64, t205: f64, t210: f64, t214: f64, t792: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1296, t1297) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk452(t1294, t763, t532, t571);
        let t1298 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk453(t514);
        let (t1301, t1302) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk454(t25, t1298, t606, t517, zeta_threshold);
        let t1307 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk455(t28, t1081, t1302, t1301, zeta_threshold);
        let (t1313, t1314) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk456(t215, t535, t782, t154, t547);
        let t1315 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk457(t1314, t205);
        let (t1317, t1322, t1323) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk458(t1307, t210, t214, t535, t792, t795, t1313, t1315);
    (t1296, t1297, t1298, t1302, t1307, t1313, t1314, t1315, t1317, t1322, t1323)
}
