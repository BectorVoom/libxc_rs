//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2501;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2502;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2503;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2504;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2505;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2506;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2507;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta666(t1088: f64, t123: f64, t50879: f64, t2394: f64, t4730: f64, t14737: f64, t690: f64, t14741: f64, t14732: f64, t3240: f64, t50884: f64, t14735: f64, t2250: f64, t11153: f64, t2244: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t50946 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2501(t1088, t123, t50879);
        let t50948 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2502(t2394, t4730);
        let t50950 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2503(t14737, t690);
        let t50952 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2504(t14741, t690);
        let t50954 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2505(t14732, t690);
        let t50957 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2506(t123, t3240, t50884);
        let (t50959, t50961) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2507(t14735, t2250, t123, t3240);
        let (t50964, t50966) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2508(t11153, t2244, t3966, t123, t3240);
    (t50946, t50948, t50950, t50952, t50954, t50957, t50959, t50961, t50964, t50966)
}
