//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 337/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk337(t1179: f64, t281: f64, t1089: f64, t1096: f64, t1101: f64, t1104: f64, t1108: f64, t1113: f64, t1118: f64, t1146: f64, t1148: f64, t1149: f64, t1158: f64, t1161: f64, t1165: f64, t1166: f64, t1169: f64, t1171: f64, t1176: f64, t145: f64, t169: f64, t242: f64, t296: f64, t299: f64, t301: f64) -> (f64, f64) {
    let t1181 = 0.01197423401025461_f64 * t281 * t1179;
    let t1182 = 0.020267214298646783_f64 * t169 * t299 * t1089 * t301 - 0.10809180959278285_f64 * t1096 + (-t1101 + 0.10611888591559791_f64 * t1104 + t1108 - 0.031835665774679375_f64 * t169 * t1113 * t242 - 0.06367133154935875_f64 * t1118 - t1146 + t1148 - 0.2133002709687175_f64 * t1149 + 0.05332506774217938_f64 * t145 * t1089) * t296 + t1158 - 0.0005811348303577384_f64 * t1161 - t1165 + 0.039914113367515366_f64 * t1166 + t1169 - 0.01197423401025461_f64 * t281 * t1171 - 0.02394846802050922_f64 * t1176 - t1181;
    (t1181, t1182)
}
