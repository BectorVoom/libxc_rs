//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 357/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk357(t1126: f64, t1133: f64, t1138: f64, t1141: f64, t1145: f64, t1151: f64, t1155: f64, t1158: f64, t1161: f64, t1167: f64, t1200: f64, t1205: f64, t1206: f64, t1228: f64, t123: f64, t1233: f64, t1234: f64, t1312: f64, t199: f64, t305: f64, t312: f64, t315: f64, t317: f64, t329: f64, t346: f64, t384: f64, t388: f64, t566: f64, t726: f64, t73: f64, t77: f64, t81: f64) -> f64 {
    let t1315 = 0.020267214298646783_f64 * t123 * t315 * t1126 * t317 - 0.10809180959278285_f64 * t1133 + t1138 - 0.0005811348303577384_f64 * t1141 - t1145 + t1151 + (-t1155 + 0.10611888591559791_f64 * t1158 + 0.10611888591559791_f64 * t1161 - 0.031835665774679375_f64 * t123 * t1167 * t199 - 0.06367133154935875_f64 * t123 * t726 * t566 - 0.031835665774679375_f64 * t123 * t305 * t1200 + t1205 - 0.2133002709687175_f64 * t1206 + 0.05332506774217938_f64 * t81 * t1126) * t312 + 3.0_f64 * t329 * t1228 + t346 * t388 * t384 + 6.0_f64 * t1233 * t77 * t1234 + t346 * t1312 * t73;
    t1315
}
