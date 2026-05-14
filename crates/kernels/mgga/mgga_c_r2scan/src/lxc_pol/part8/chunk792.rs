//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 792/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk792<F: Float>(t4741: F, t5309: F, t5312: F, t5315: F) -> (F, F) {
    let t5860 = 0.32547666666666666667e-1 * t4741;
    let t5861 = -0.14816666666666666667e-1 * t5309 + 0.9877777777777777778e-2 * t5312 - 0.46096296296296296297e-1 * t5315 - t5860;
    (t5860, t5861)
}
