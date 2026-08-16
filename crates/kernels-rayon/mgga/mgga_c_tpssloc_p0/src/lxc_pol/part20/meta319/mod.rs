//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1582;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1583;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1584;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta319(t135: f64, t3471: f64, t1174: f64, t11168: f64, t4908: f64, t11159: f64, t4900: f64, t1184: f64, t4899: f64, t3242: f64, t460: f64, t2244: f64, t3448: f64, t3469: f64, t3451: f64, t2250: f64, t3450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11560, t11561, t11563, t11566, t11569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1582(t135, t3471, t1174, t11168, t4908, t11159, t4900, t1184, t4899);
        let t11570 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1583(t3242, t460);
        let t11571 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1584(t11570, t2244);
        let (t11572, t11575, t11576, t11579) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1585(t11569, t11571, t3448, t3469, t3451, t2250, t3450);
    (t11560, t11561, t11563, t11566, t11569, t11570, t11571, t11572, t11575, t11576, t11579)
}
