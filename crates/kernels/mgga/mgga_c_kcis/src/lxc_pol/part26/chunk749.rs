//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 749/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk749<F: Float>(t9062: F, t9066: F, t9150: F, t9152: F, t9155: F, t9158: F, t9163: F, t9166: F, t9168: F, t9170: F, t9173: F, t9176: F, t9179: F, t9182: F, t9296: F, t160: F, t167: F) -> (F, F) {
    let t9311 = -t9062 / 8.0 - 3.0 / 4.0 * t9066 + t9150 / 8.0 - 3.0 / 8.0 * t9152 + 3.0 / 32.0 * t9155 + t9158 / 64.0 + 15.0 / 8.0 * t9163 - 3.0 / 2.0 * t9166 - 3.0 / 4.0 * t9168 + 3.0 / 64.0 * t9170 + 3.0 / 4.0 * t9173 - t9176 / 64.0 + 3.0 / 8.0 * t9179 - 3.0 / 8.0 * t9182;
    let t9312 = t9296 + t9311;
    let t9323 = t167 * t160;
    (t9312, t9323)
}
