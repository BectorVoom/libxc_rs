//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta108 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk744;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk745;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk746;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk747;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk748;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk749;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk750;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta108(t225: f64, t2666: f64, t68: f64, t845: f64, t2379: f64, t2553: f64, t824: f64, t228: f64, t230: f64, t822: f64, t825: f64, t232: f64, t819: f64, t820: f64, t2631: f64, t20: f64, t61: f64, t241: f64, t244: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2667, t2671, t2672, t2675, t2678) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk744(t225, t2666, t68, t845, t2379, t2553, t824, t228, t230, t822, t825);
        let t2679 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk745(t232, t2678);
        let t2681 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk746(t2679, t819, t820);
        let t2684 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk747(t232, t2631);
        let t2686 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk748(t2684, t819, t820);
        let t2690 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk749(t20, t61);
        let t2691 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk750(t241, t2690);
        let t2693 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk751(t244, t248, t2691);
    (t2667, t2671, t2672, t2675, t2678, t2679, t2681, t2684, t2686, t2690, t2691, t2693)
}
