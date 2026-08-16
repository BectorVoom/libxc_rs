//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1311;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1312;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta275(t19: f64, t9223: f64, t2233: f64, t604: f64, t2239: f64, t601: f64, t83: f64, t84: f64, t85: f64, t24: f64, t41: f64, t42: f64, t53: f64, t54: f64, t2585: f64, t2769: f64, t73: f64, t3241: f64, t76: f64, t111: f64, t2311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9225, t9228, t9231) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1311(t19, t9223, t2233, t604, t2239, t601);
        let (t9238, t9239) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1312(t83, t84, t85, t24);
        let (t9287, t9300, t9311, t9321, t9330, t9348) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1313(t41, t42, t53, t54, t2585, t2769, t73, t3241, t76, t111, t2311);
    (t9225, t9228, t9231, t9238, t9239, t9287, t9300, t9311, t9321, t9330, t9348)
}
