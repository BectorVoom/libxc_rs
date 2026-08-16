//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta7 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk48;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk49;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk50;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk51;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk52;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk53;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk54;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk55;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk56;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk57;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta7(t107: f64, t64: f64, t89: f64, t25: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t67: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t111, t109) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk48(t107, t64);
        let t112 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk49(t111);
        let t113 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk50(t112, t89);
        let t116 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk51(t25, dens_threshold, rho0, zeta_threshold);
        let t117 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk52(t116);
        let t118 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk53(t117, t67);
        let t119 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk54();
        let t120 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk55(t119);
        let t121 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk56(t60);
        let (t122, t123) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk57(t120, t121, t118);
    (t111, t109, t112, t113, t116, t117, t118, t119, t120, t121, t122, t123)
}
