//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 715/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk715<F: Float>(t809: F, t9181: F, t9062: F, t9066: F, t9150: F, t9152: F, t9155: F, t9158: F, t9163: F, t9166: F, t9168: F, t9170: F, t9173: F, t9176: F, t9179: F, t9060: F) -> (F, F) {
    let t9182 = t9181 * t809;
    let t9184 = -0.1875e0 * t9062 - 0.1125e1 * t9066 + 0.1875e0 * t9150 - 0.5625e0 * t9152 + 0.2428125e0 * t9155 + 0.4046875e-1 * t9158 + 0.485625e1 * t9163 - 0.225e1 * t9166 - 0.1125e1 * t9168 + 0.12140625e0 * t9170 + 0.1125e1 * t9173 - 0.4046875e-1 * t9176 + 0.97125e0 * t9179 - 0.5625e0 * t9182;
    let t9185 = t9060 + t9184;
    (t9182, t9185)
}
