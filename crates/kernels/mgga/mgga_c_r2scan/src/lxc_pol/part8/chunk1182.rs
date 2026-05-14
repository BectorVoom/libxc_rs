//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1182/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1182<F: Float>(t166: F, t22644: F, t759: F, t2288: F, t58: F, t597: F, t6007: F, t423: F, t2055: F, t6038: F, t761: F, t6045: F, t2049: F, t2056: F, t158: F, t6817: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22647 = 0.285764e-1 * t759 * t166 * t22644;
    let t22648 = t2288 * t58;
    let t22650 = t597 * t6007;
    let t22651 = t22648 * t423 * t22650;
    let t22661 = t2055 * t6038 * t761;
    let t22665 = 0.2286112e0 * t2055 * t6045 * t761;
    let t22666 = t2049 * t2049;
    let t22669 = 0.1714584e0 * t2055 * t166 * t22666;
    let t22671 = t2056 * t2056;
    let t22674 = 0.6858336e0 * t6817 * t158 * t166 * t22671;
    (t22647, t22648, t22650, t22651, t22661, t22665, t22666, t22669, t22671, t22674)
}
