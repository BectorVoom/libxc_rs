//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2151/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2151<F: Float>(t15125: F, t15191: F, t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F) {
    let t15638 = F::cast_from(0.19755555555555555556e-1_f64) * t15125;
    let t15639 = F::cast_from(0.9877777777777777778e-2_f64) * t15191;
    let t15648 = -t11133 - F::cast_from(0.13170370370370370371e-1_f64) * t11134 + F::cast_from(0.32925925925925925927e-2_f64) * t11136 - F::cast_from(0.9877777777777777778e-2_f64) * t11138 + F::cast_from(0.4938888888888888889e-2_f64) * t11140 - F::cast_from(0.65851851851851851853e-2_f64) * t15189 + F::cast_from(0.65851851851851851854e-2_f64) * t15127 - t15638 + t15639 - F::cast_from(0.16462962962962962963e-1_f64) * t15142 + F::cast_from(0.59266666666666666668e-1_f64) * t15156 - F::cast_from(0.19755555555555555556e-1_f64) * t15132 - F::cast_from(0.9877777777777777778e-2_f64) * t15137 - F::cast_from(0.88900000000000000002e-1_f64) * t15160 + F::cast_from(0.59266666666666666668e-1_f64) * t15147 + F::cast_from(0.29633333333333333334e-1_f64) * t15151 - F::cast_from(0.14816666666666666667e-1_f64) * t15195;
    (t15638, t15639, t15648)
}
