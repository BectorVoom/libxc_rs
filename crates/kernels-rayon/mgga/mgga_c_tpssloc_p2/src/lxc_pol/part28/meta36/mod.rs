//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta36 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk252;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk253;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk254;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk255;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk256;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta36(t123: f64, t67: f64, t687: f64, t3: f64, t61: f64, t119: f64, t133: f64, t688: f64, t690: f64, t141: f64, t683: f64, t31: f64, t32: f64, t152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t693, t694, t697, t698) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk252(t123, t67, t687, t3, t61, t119);
        let t699 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk253(t133, t698);
        let (t701, t702) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk254(t688, t690, t694, t699, t141);
        let (t703, t705) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk255(t701, t702, t683);
        let t706 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk256(t31, t32);
        let t707 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk257(t152, t706);
    (t693, t694, t697, t698, t699, t701, t702, t703, t705, t706, t707)
}
