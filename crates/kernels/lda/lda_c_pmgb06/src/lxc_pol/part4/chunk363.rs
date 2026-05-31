//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 363/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk363<F: Float>(t1289: F, t1291: F, t1296: F, t1297: F, t1309: F, t378: F, t384: F, t74: F, t387: F, t1126: F, t1133: F, t1138: F, t1141: F, t1145: F, t1151: F, t1155: F, t1158: F, t1161: F, t1167: F, t1200: F, t1205: F, t1206: F, t1228: F, t123: F, t1233: F, t1234: F, t199: F, t305: F, t312: F, t315: F, t317: F, t329: F, t346: F, t388: F, t566: F, t726: F, t73: F, t77: F, t81: F) -> (F, F) {
    let t1311 = t1289 * t74 - F::cast_from(2.0_f64) * t1291 * t384 + F::cast_from(2.0_f64) * t1296 * t1297 - t378 * t1309;
    let t1312 = t1311 * t387;
    let t1315 = F::cast_from(0.020267214298646783_f64) * t123 * t315 * t1126 * t317 - F::cast_from(0.10809180959278285_f64) * t1133 + t1138 - F::cast_from(0.0005811348303577384_f64) * t1141 - t1145 + t1151 + (-t1155 + F::cast_from(0.10611888591559791_f64) * t1158 + F::cast_from(0.10611888591559791_f64) * t1161 - F::cast_from(0.031835665774679375_f64) * t123 * t1167 * t199 - F::cast_from(0.06367133154935875_f64) * t123 * t726 * t566 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t1200 + t1205 - F::cast_from(0.2133002709687175_f64) * t1206 + F::cast_from(0.05332506774217938_f64) * t81 * t1126) * t312 + F::cast_from(3.0_f64) * t329 * t1228 + t346 * t388 * t384 + F::cast_from(6.0_f64) * t1233 * t77 * t1234 + t346 * t1312 * t73;
    (t1312, t1315)
}
