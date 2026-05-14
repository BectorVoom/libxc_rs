//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1284/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1284<F: Float>(t19797: F, t19801: F, t24049: F, t24052: F, t24053: F, t24055: F, t24061: F, t24066: F, t24068: F, t24072: F, t24076: F, t24077: F, t24080: F, t2667: F, t6568: F, t2191: F, t7245: F) -> (F, F) {
    let t24084 = 0.19776387377308997907e1 * t24049 - t24052 + 0.10401866088065122276e1 * t24053 - 0.89443204944342177673e-3 * t24055 - 0.43341108700271342816e-1 * t2667 * t6568 + 0.82318114786693894983e-2 * t24061 - 0.1047928639570397803e0 * t24066 - 0.52690178912667028301e0 * t24068 - 0.16463622957338778996e-1 * t24072 - t24076 - 0.34930954652346593433e-1 * t24077 - 0.1047928639570397803e0 * t24080 - 0.22084125774650235183e1 * t19797 - 0.66252377323950705547e1 * t19801;
    let t24086 = t7245 * t2191;
    (t24084, t24086)
}
