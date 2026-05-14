//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1045/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1045<F: Float>(t3446: F, t3453: F, t7136: F, t2259: F, t3574: F, t113: F, t36985: F, t97: F, t10810: F, t3429: F, t3692: F, t10935: F, t2816: F, t10928: F, t122: F, t3434: F, t874: F, t955: F) -> (F, F, F, F, F, F) {
    let t40521 = t3446 * t3453 * t7136;
    let t40523 = t3574 * t2259;
    let t40549 = t97 * t36985 * t113;
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    (t40521, t40523, t40549, t40556, t40559, t40564)
}
