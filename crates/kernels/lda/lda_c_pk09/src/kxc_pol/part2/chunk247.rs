//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 247/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk247<F: Float>(t1067: F, t90: F, t106: F, t115: F, t1007: F, t1011: F, t1026: F, t1041: F, t1047: F, t1052: F, t1059: F, t1065: F, t653: F, t709: F, t713: F, t757: F, t933: F, t98: F, t993: F, t994: F) -> (F, F, F, F) {
    let t1069 = t90 * t1067 / F::cast_from(9.0_f64);
    let t1071 = t106 * t1067 / F::cast_from(9.0_f64);
    let t1073 = t115 * t1067 / F::cast_from(9.0_f64);
    let t1075 = -t993 - t994 - t1007 * t98 / F::cast_from(6.0_f64) - t106 * t1011 / F::cast_from(6.0_f64) - t1026 * t98 / F::cast_from(6.0_f64) + t1041 * t98 / F::cast_from(6.0_f64) + t115 * t1011 / F::cast_from(6.0_f64) + t933 * t1047 / F::cast_from(36.0_f64) - t90 * t1011 / F::cast_from(6.0_f64) + t1052 * t713 / F::cast_from(6.0_f64) + t1052 * t709 / F::cast_from(6.0_f64) - t1059 * t98 / F::cast_from(6.0_f64) + t1065 + F::cast_from(0.10237773105191754_f64) * t653 + t1069 + t1071 - t1073 + F::cast_from(0.14975624337724558_f64) * t757;
    (t1069, t1071, t1073, t1075)
}
