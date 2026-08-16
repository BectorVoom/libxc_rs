//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 573/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk573<F: Float>(t1004: F, t1040: F, t1013: F, t361: F, t363: F, t3037: F, t3033: F) -> (F, F, F, F, F) {
    let t3117 = t1004 * t1040;
    let t3127 = F::cast_from(1.0_f64) / t1013 / t361;
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    (t3117, t3127, t3128, t3129, t3130)
}
