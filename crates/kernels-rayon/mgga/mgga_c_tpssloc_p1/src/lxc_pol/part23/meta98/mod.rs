//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk549;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk550;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk551;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk552;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta98(t1089: f64, t460: f64, t3247: f64, t461: f64, t3293: f64, t3030: f64, t466: f64, t3032: f64, t1208: f64, t476: f64, t478: f64, t3036: f64, t483: f64, t475: f64, t1210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3450, t3455, t3464, t3499, t3500) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk549(t1089, t460, t3247, t461, t3293, t3030, t466, t3032);
        let (t3502, t3503) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk550(t1208, t476, t478);
        let (t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk551(t3036, t483, t3503, t3500);
        let t3508 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk552(t475);
        let (t3514, t3515) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk553(t1210, t3504, t3500);
    (t3450, t3455, t3464, t3499, t3500, t3502, t3503, t3505, t3506, t3508, t3514, t3515)
}
