//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1108;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1109;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1110;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta187(t103: f64, t5484: f64, t100: f64, t104: f64, t1447: f64, t1450: f64, t5469: f64, t5472: f64, t5475: f64, t5481: f64, t92: f64, t109: f64, t656: f64, t2327: f64, t4041: f64, t5465: f64, t64: f64, t510: f64, t40: f64, t4100: f64, t4102: f64, t185: f64, t5392: f64, t2658: f64, t1484: f64, t4310: f64, t1462: f64, t4205: f64, t2433: f64, t5398: f64, t73: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5485, t5488) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1108(t103, t5484, t100, t104, t1447, t1450, t5469, t5472, t5475, t5481, t92);
        let (t5489, t5493) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1109(t109, t5488, t656, t2327, t4041, t5465, t64);
        let t5494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1110(t510, t5493);
        let (t5497, t5498, t5499, t5501, t5502, t5506, t5512) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1111(t40, t4100, t4102, t185, t5392, t2658, t1484, t4310, t1462, t4205, t2433, t5398, t73, zeta_threshold);
    (t5485, t5488, t5489, t5493, t5494, t5497, t5498, t5499, t5501, t5502, t5506, t5512)
}
