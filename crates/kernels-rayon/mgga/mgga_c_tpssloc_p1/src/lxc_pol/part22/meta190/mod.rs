//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1121;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1122;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1123;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1124;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1125;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1126;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta190(t2645: f64, t4181: f64, t5591: f64, t4212: f64, t185: f64, t5398: f64, t707: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t5497: f64, t5498: f64, t5501: f64, t5506: f64, t5521: f64, t5524: f64, t5525: f64, t225: f64, t2671: f64, t5527: f64, t5544: f64, t824: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t232: f64, t819: f64, t820: f64, t5584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5593 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1121(t2645, t4181, t5591);
        let (t5596, t5597, t5599, t5600) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1122(t4212, t185, t5398, t707, t2373, t2377, t2408, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t5497, t5498, t5501, t5506, t5521, t5524, t5525);
        let (t5601, t5605, t5608, t5611) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1123(t225, t5600, t2671, t5527, t5544, t824, t1504, t1506, t228, t230);
        let t5612 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1124(t232, t5611);
        let t5614 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1125(t5612, t819, t820);
        let t5617 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1126(t232, t5584);
        let t5619 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1127(t5617, t819, t820);
    (t5593, t5596, t5597, t5599, t5601, t5605, t5608, t5611, t5612, t5614, t5617, t5619)
}
