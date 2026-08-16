//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk435;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk436;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk437;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk438;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk439;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta66(t1287: f64, t17: f64, t1284: f64, t182: f64, t521: f64, t67: f64, t758: f64, t172: f64, t763: f64, t532: f64, t571: f64, t514: f64, t25: f64, t606: f64, t517: f64, zeta_threshold: f64, t28: f64, t1081: f64, t215: f64, t535: f64, t782: f64, t154: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1288, t1290, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk435(t1287, t17, t1284, t182, t521, t67, t758, t172);
        let (t1296, t1297) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk436(t1294, t763, t532, t571);
        let t1298 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk437(t514);
        let (t1301, t1302) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk438(t25, t1298, t606, t517, zeta_threshold);
        let t1307 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk439(t28, t1081, t1302, t1301, zeta_threshold);
        let (t1313, t1314) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk440(t215, t535, t782, t154, t547);
    (t1288, t1290, t1291, t1293, t1294, t1296, t1297, t1298, t1302, t1307, t1313, t1314)
}
