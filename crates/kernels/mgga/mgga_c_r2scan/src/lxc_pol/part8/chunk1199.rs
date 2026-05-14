//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1199/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1199<F: Float>(t20837: F, t7297: F, t2666: F, t6527: F, t20481: F, t549: F, t551: F, t938: F, t2699: F, t6512: F, t21028: F, t253: F, t5134: F, t6407: F, t7407: F, t2162: F, t2228: F) -> (F, F, F, F, F, F, F, F) {
    let t24018 = t20837 * t7297;
    let t24028 = t6527 * t2666;
    let t24049 = t549 * t551 * t20481 * t938;
    let t24051 = t6512 * t2699;
    let t24052 = 0.12805040077930161442e1 * t24051;
    let t24055 = t21028 * t7297;
    let t24063 = t5134 * t253;
    let t24068 = t6407 * t7407;
    let t24074 = t2228 * t2162;
    (t24018, t24028, t24049, t24052, t24055, t24063, t24068, t24074)
}
