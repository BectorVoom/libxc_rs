//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 844/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk844<F: Float>(t11: F, t5: F, t5193: F, t5195: F, t5198: F, t5991: F, t5997: F, t6004: F, t6020: F, t6048: F, t146: F, t147: F, t122: F, t1415: F, t2111: F, t2117: F, t57: F) -> (F, F, F) {
    let t6053 = -t5193 + 20.0 / 3.0 * t5195 - 5.0 * t5198 + 5.0 * t5 * t11 * t5991 - 45.0 * param_eta * (t5997 + t6004 + t6020 + t6048);
    let t6055 = t146 * t147 * t6053;
    let t6062 = 0.1590300183910403919e-2 * t2111 * t122 * t1415 * t57 * t2117;
    (t6053, t6055, t6062)
}
