//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1369/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1369<F: Float>(t25966: F, t565: F, t25767: F, t25962: F, t2719: F, t549: F, t551: F, t6343: F, t2207: F, t2691: F, t6416: F, t11747: F, t545: F, t6167: F, t22709: F, t5108: F, t7352: F) -> (F, F, F, F, F) {
    let t25972 = t565 * t25966;
    let t25974 = t25972 * t25767 * t25962;
    let t25978 = t549 * t551 * t6343 * t2719;
    let t25979 = 0.12713391885412927226e1 * t25978;
    let t25981 = t2207 * t6416 * t2691;
    let t25983 = t545 * t11747;
    let t25984 = t25983 * t6167;
    let t25990 = t5108 * t22709 * t7352;
    (t25974, t25979, t25981, t25984, t25990)
}
