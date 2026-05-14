//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1282/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1282<F: Float>(t20481: F, t549: F, t551: F, t938: F, t2699: F, t6512: F, t2731: F, t6490: F, t21028: F, t7297: F, t113: F, t7202: F, t538: F, t7623: F, t253: F, t5134: F) -> (F, F, F, F, F, F, F) {
    let t24049 = t549 * t551 * t20481 * t938;
    let t24051 = t6512 * t2699;
    let t24052 = 0.12805040077930161442e1 * t24051;
    let t24053 = t6490 * t2731;
    let t24055 = t21028 * t7297;
    let t24059 = t7202 * t113;
    let t24061 = t7623 * t538 * t24059;
    let t24063 = t5134 * t253;
    (t24049, t24052, t24053, t24055, t24059, t24061, t24063)
}
