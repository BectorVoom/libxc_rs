//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 333/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk333<F: Float>(t1159: F, t159: F, t285: F, t477: F, t695: F, t684: F, t688: F, t692: F, t1112: F, t465: F, t281: F, t1128: F, t147: F, t1089: F, t1096: F, t1101: F, t1104: F, t1108: F, t1113: F, t1118: F, t1146: F, t1148: F, t1149: F, t1158: F, t145: F, t169: F, t242: F, t296: F, t299: F, t301: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1161 = t1159 * t159 * t285;
    let t1165 = 0.0005811348303577384 * t695 * t477 * t285;
    let t1166 = t684 * t688;
    let t1169 = 0.039914113367515366 * t684 * t692;
    let t1171 = t1112 * t159 * t285;
    let t1175 = t465 * t477 * t285;
    let t1176 = t281 * t1175;
    let t1179 = t147 * t1128 * t285;
    let t1181 = 0.01197423401025461 * t281 * t1179;
    let t1182 = 0.020267214298646783 * t169 * t299 * t1089 * t301 - 0.10809180959278285 * t1096 + (-t1101 + 0.10611888591559791 * t1104 + t1108 - 0.031835665774679375 * t169 * t1113 * t242 - 0.06367133154935875 * t1118 - t1146 + t1148 - 0.2133002709687175 * t1149 + 0.05332506774217938 * t145 * t1089) * t296 + t1158 - 0.0005811348303577384 * t1161 - t1165 + 0.039914113367515366 * t1166 + t1169 - 0.01197423401025461 * t281 * t1171 - 0.02394846802050922 * t1176 - t1181;
    (t1161, t1165, t1166, t1169, t1171, t1175, t1176, t1179, t1181, t1182)
}
