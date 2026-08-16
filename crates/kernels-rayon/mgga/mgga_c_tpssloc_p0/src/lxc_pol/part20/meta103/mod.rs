//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta103 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk694;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk695;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta103(t2563: f64, t789: f64, t59: f64, t591: f64, t207: f64, t795: f64, t154: f64, t244: f64, t205: f64, t210: f64, t214: f64, t2379: f64, t786: f64, t792: f64, t118: f64, t776: f64, t794: f64, t2553: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2564, t2566) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk691(t2563, t789, t59, t591);
        let (t2569, t2570) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk692(t207, t2566, t795, t154, t244);
        let t2571 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk693(t205, t2570);
        let (t2573, t2576) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk694(t210, t214, t2379, t786, t792);
        let (t2578, t2579, t2582, t2585) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk695(t118, t776, t794, t2576, t210, t214, t2553, t59, t835);
        let t2586 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk696(t154, t2585);
    (t2564, t2566, t2569, t2570, t2571, t2573, t2576, t2578, t2579, t2582, t2585, t2586)
}
