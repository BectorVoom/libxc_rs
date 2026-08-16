//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta101 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk687;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk688;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk689;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk690;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk691;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk692;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk693;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta101(t244: f64, t248: f64, t2691: f64, t238: f64, t835: f64, t841: f64, t812: f64, t849: f64, t1891: f64, t241: f64, t67: f64, t225: f64, t853: f64, t257: f64, t856: f64, t68: f64, t252: f64, t2627: f64, t814: f64, t852: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2693 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk687(t244, t248, t2691);
        let (t2695, t2696) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk688(t238, t2693, t835, t841);
        let t2697 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk689(t2696, t812);
        let (t2698, t2701) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk690(t2697, t849, t1891, t241, t67);
        let t2713 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk691(t225, t853);
        let t2718 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk692(t257, t856, t68);
        let (t2728, t2732) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk693(t252, t2627, t814, t852);
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk694(t261);
    (t2693, t2695, t2696, t2697, t2698, t2701, t2713, t2718, t2728, t2732, t2751, t2752)
}
