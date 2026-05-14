//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 777/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk777<F: Float>(t1103: F, t3279: F, t6272: F, t1104: F, t6276: F, t3288: F, t6320: F, t345: F, t1727: F, t4606: F, t3293: F, t1109: F, t6338: F, t3303: F, t6316: F, t1114: F, t6352: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6582 = t1103 * t3279 * t6272;
    let t6586 = t1103 * t1104 * t6276;
    let t6589 = t3288 * t6320;
    let t6590 = t345 * t6589;
    let t6593 = t4606 * t1727;
    let t6594 = t3293 * t6593;
    let t6597 = t1109 * t6338;
    let t6598 = t345 * t6597;
    let t6601 = t3303 * t6316;
    let t6602 = t345 * t6601;
    let t6605 = t1114 * t6352;
    (t6582, t6586, t6589, t6590, t6593, t6594, t6597, t6598, t6601, t6602, t6605)
}
