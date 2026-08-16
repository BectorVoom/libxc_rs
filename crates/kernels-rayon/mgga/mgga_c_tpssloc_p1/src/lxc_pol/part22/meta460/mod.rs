//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1833;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1834;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1835;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1836;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta460(t25: f64, t19593: f64, t1408: f64, t6305: f64, t12061: f64, t20216: f64, t5134: f64, t514: f64, t5397: f64, t1649: f64, t6312: f64, zeta_threshold: f64, t28: f64, t12072: f64, t5142: f64, t517: f64, t5966: f64, t157: f64, t182: f64, t11987: f64, t1298: f64, t5170: f64, t12000: f64, t1302: f64, t5178: f64, t1807: f64, t6434: f64, t12351: f64, t20356: f64, t820: f64, t1825: f64, t19956: f64, t5248: f64, t550: f64, t6330: f64, t12419: f64, t5249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20372, t20376, t20384, t20385, t20390) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1833(t25, t19593, t1408, t6305, t12061, t20216, t5134, t514, t5397, t1649, t6312, zeta_threshold);
        let t20396 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1834(t28, t12072, t20385, t20390, t5142, t517, t5966, t157, t20384, zeta_threshold);
        let (t20398, t20406, t20414) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1835(t25, t28, t182, t20396, t11987, t1298, t20216, t20376, t5170, t5397, t12000, t1302, t20385, t20390, t5178, t5966, zeta_threshold);
        let t20416 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1836(t20406, t20414);
        let (t20420, t20433, t20442, t20448, t20450) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1837(t1807, t6434, t12351, t20356, t820, t1825, t19956, t5248, t550, t6330, t12419, t5249);
    (t20372, t20376, t20385, t20390, t20396, t20398, t20416, t20420, t20433, t20442, t20448, t20450)
}
