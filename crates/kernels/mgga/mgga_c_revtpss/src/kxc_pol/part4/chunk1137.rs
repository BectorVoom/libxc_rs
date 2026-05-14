//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1137/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1137<F: Float>(t15125: F, t15191: F, t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F,) {
    let t15638 = 0.19755555555555555556e-1 * t15125;
    let t15639 = 0.9877777777777777778e-2 * t15191;
    let t15648 = -t11133 - 0.13170370370370370371e-1 * t11134 + 0.32925925925925925927e-2 * t11136 - 0.9877777777777777778e-2 * t11138 + 0.4938888888888888889e-2 * t11140 - 0.65851851851851851853e-2 * t15189 + 0.65851851851851851854e-2 * t15127 - t15638 + t15639 - 0.16462962962962962963e-1 * t15142 + 0.59266666666666666668e-1 * t15156 - 0.19755555555555555556e-1 * t15132 - 0.9877777777777777778e-2 * t15137 - 0.88900000000000000002e-1 * t15160 + 0.59266666666666666668e-1 * t15147 + 0.29633333333333333334e-1 * t15151 - 0.14816666666666666667e-1 * t15195;
    (t15648,)
}
