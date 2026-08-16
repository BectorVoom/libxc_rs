//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta108 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk719;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk720;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk721;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk722;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk723;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk724;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk725;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk726;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta108(t68: f64, t845: f64, t2379: f64, t2553: f64, t824: f64, t228: f64, t230: f64, t2667: f64, t822: f64, t825: f64, t232: f64, t819: f64, t820: f64, t2631: f64, t20: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64, t835: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2672, t2675, t2678) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk719(t68, t845, t2379, t2553, t824, t228, t230, t2667, t822, t825);
        let t2679 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk720(t232, t2678);
        let t2681 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk721(t2679, t819, t820);
        let t2684 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk722(t232, t2631);
        let t2686 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk723(t2684, t819, t820);
        let t2690 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk724(t20, t61);
        let t2691 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk725(t241, t2690);
        let t2693 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk726(t244, t248, t2691);
        let (t2695, t2696) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk727(t238, t2693, t835, t841);
    (t2672, t2675, t2678, t2679, t2681, t2684, t2686, t2690, t2691, t2693, t2695, t2696)
}
