//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta7 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk47;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk48;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk49;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk50;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk51;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk52;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk53;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk54;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk55;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta7(t101: f64, t103: f64, t100: f64, t92: f64, t96: f64, t64: f64, t89: f64, t25: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104, t106, t107) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk47(t101, t103, t100, t92, t96);
        let (t111, t109) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk48(t107, t64);
        let t112 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk49(t111);
        let t113 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk50(t112, t89);
        let t116 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk51(t25, dens_threshold, rho0, zeta_threshold);
        let t117 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk52(t116);
        let t118 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk53(t117, t67);
        let t119 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk54();
        let t120 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk55(t119);
    (t104, t106, t107, t111, t109, t112, t113, t116, t117, t118, t119, t120)
}
