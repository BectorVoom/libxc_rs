//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk453;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk454;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk455;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk456;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta77(t2409: f64, t125: f64, t701: f64, t141: f64, t138: f64, t681: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2410, t2411) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk453(t2409, t125);
        let t2412 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk454(t701);
        let (t2413, t2414) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk455(t141);
        let (t2415, t2417) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk456(t2412, t2414, t2411);
        let (t2418, t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk457(t138, t681, t125, t2412, t702);
    (t2410, t2411, t2412, t2413, t2414, t2415, t2417, t2418, t2419, t2420, t2421, t2423)
}
