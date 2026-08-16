//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk559;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk560;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk561;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk562;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta100(t1089: f64, t415: f64, t61: f64, t1239: f64, t496: f64, t68: f64, t3032: f64, t3502: f64, t3499: f64, t1932: f64, t3508: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3584 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk559(t1089, t415);
        let t3585 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk560(t3584, t61);
        let (t3598, t3609, t3610) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk561(t1239, t496, t68, t3032, t3502, t3499);
        let t3612 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk562(t1932, t3508);
        let (t3623, t3624) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk563(t1209, t3032, t3499);
    (t3584, t3585, t3598, t3609, t3610, t3612, t3623, t3624)
}
