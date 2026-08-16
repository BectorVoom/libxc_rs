//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta7 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk53;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk54;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk55;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk56;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk57;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk58;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk59;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk60;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk61;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta7(t117: f64, t67: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t118 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk53(t117, t67);
        let t119 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk54();
        let t120 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk55(t119);
        let t121 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk56(t60);
        let (t122, t123) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk57(t120, t121, t118);
        let t125 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk58(t123);
        let t126 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk59(t123);
        let (t129, t131) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk60(t123, t67);
        let (t132, t133) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk61(t117, t131);
    (t118, t119, t120, t121, t122, t123, t125, t126, t129, t131, t132, t133)
}
