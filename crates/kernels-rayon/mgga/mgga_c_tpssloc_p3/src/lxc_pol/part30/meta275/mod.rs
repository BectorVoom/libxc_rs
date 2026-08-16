//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1245;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1246;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1247;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1248;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta275(t1530: f64, t25: f64, t1408: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7476: f64, t7541: f64, t1539: f64, t6690: f64, t6689: f64, t1599: f64, t1922: f64, t1625: f64, t225: f64, t387: f64, t345: f64, t1634: f64, t6705: f64, t6704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7545, t7552, t7553) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1245(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1539, t6690);
        let t7554 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1246(t6689, t7553);
        let t7557 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1247(t1599, t1922);
        let (t7560, t7561) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1248(t1625, t225, t387);
        let (t7562, t7565, t7566) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1249(t345, t7561, t1634, t6705, t6704);
    (t7545, t7552, t7553, t7554, t7557, t7560, t7561, t7562, t7565, t7566)
}
