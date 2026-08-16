//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta266 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1273;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1274;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1275;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1276;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1277;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1278;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta266(t1528: f64, t1912: f64, t259: f64, t4147: f64, t4268: f64, t6549: f64, t6565: f64, t6627: f64, t7481: f64, t7486: f64, t7490: f64, t7492: f64, t7511: f64, t7517: f64, t7538: f64, t855: f64, t870: f64, t1530: f64, t25: f64, t1408: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7476: f64, t1539: f64, t6690: f64, t6689: f64, t1599: f64, t1922: f64, t1625: f64, t225: f64, t387: f64, t345: f64, t1634: f64, t6705: f64, t6704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7540 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1273(t1528, t1912, t259, t4147, t4268, t6549, t6565, t6627, t7481, t7486, t7490, t7492, t7511, t7517, t7538, t855);
        let t7541 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1274(t7540, t870);
        let (t7545, t7552, t7553) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1275(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1539, t6690);
        let t7554 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1276(t6689, t7553);
        let t7557 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1277(t1599, t1922);
        let (t7560, t7561) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1278(t1625, t225, t387);
        let (t7562, t7565, t7566) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1279(t345, t7561, t1634, t6705, t6704);
    (t7540, t7541, t7545, t7552, t7553, t7554, t7557, t7560, t7561, t7562, t7565, t7566)
}
