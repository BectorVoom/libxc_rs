//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1114/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1114<F: Float>(t122: F, t6159: F, t6161: F, t5149: F, t2195: F, t6474: F, t20137: F, t6480: F, t6481: F, t2097: F, t2167: F, t546: F, t565: F, t110: F, t4145: F, t524: F) -> (F, F, F, F, F, F, F) {
    let t20837 = t6161 * t6159 * t122;
    let t20838 = t20837 * t5149;
    let t20852 = t2195 * t6474;
    let t20858 = t6480 * t20137 * t6481;
    let t20860 = t2167 * t2097;
    let t20861 = t546 * t20860;
    let t20864 = t565 * t20860;
    let t20868 = t524 * t4145 * t110;
    (t20837, t20838, t20852, t20858, t20861, t20864, t20868)
}
