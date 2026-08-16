//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk939;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk940;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk941;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk942;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta267<F: Float>(t25: F, t19593: F, t1408: F, t6305: F, t12061: F, t20216: F, t5134: F, t514: F, t5397: F, t1649: F, t6312: F, zeta_threshold: F, t28: F, t12072: F, t5142: F, t517: F, t5966: F, t157: F, t182: F, t11987: F, t1298: F, t5170: F, t12000: F, t1302: F, t5178: F, t1807: F, t6434: F, t12351: F, t20356: F, t820: F, t1825: F, t19956: F, t5248: F, t550: F, t6330: F, t12419: F, t5249: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20372, t20376, t20384, t20385, t20390) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk939::<F>(t25, t19593, t1408, t6305, t12061, t20216, t5134, t514, t5397, t1649, t6312, zeta_threshold);
        let t20396 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk940::<F>(t28, t12072, t20385, t20390, t5142, t517, t5966, t157, t20384, zeta_threshold);
        let (t20398, t20406, t20414) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk941::<F>(t25, t28, t182, t20396, t11987, t1298, t20216, t20376, t5170, t5397, t12000, t1302, t20385, t20390, t5178, t5966, zeta_threshold);
        let t20416 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk942::<F>(t20406, t20414);
        let (t20420, t20433, t20442, t20448, t20450) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk943::<F>(t1807, t6434, t12351, t20356, t820, t1825, t19956, t5248, t550, t6330, t12419, t5249);
    (t20372, t20390, t20396, t20398, t20416, t20420, t20433, t20442, t20448, t20450)
}
