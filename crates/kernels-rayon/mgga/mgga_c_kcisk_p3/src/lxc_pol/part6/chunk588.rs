//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 588/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk588(t1471: f64, t4282: f64, t7706: f64, t1472: f64, t7710: f64, t416: f64, t8159: f64, t140: f64, t1470: f64, t2221: f64, t2225: f64, t2242: f64, t4253: f64, t4264: f64, t460: f64, t476: f64, t479: f64, t6275: f64, t6296: f64, t7865: f64, t7869: f64, t7873: f64, t7878: f64, t7898: f64, t8192: f64, t8212: f64, t8216: f64) -> (f64, f64, f64, f64) {
    let t8220 = t1471 * t4282 * t7706;
    let t8224 = t1471 * t1472 * t7710;
    let t8227 = t416 * t8159;
    let t8231 = 0.619125e-2_f64 * t8192 * t460 + 0.1857375e-1_f64 * t2242 * t2221 - 0.123825e-1_f64 * t2242 * t2225 + 0.46434375e-2_f64 * t476 * t7865 - 0.1857375e-1_f64 * t4253 * t7869 + 0.9286875e-2_f64 * t476 * t7873 + 0.123825e-1_f64 * t476 * t7878 - 0.619125e-2_f64 * t476 * t7898 + t4264 - 0.35374814814814814814e-1_f64 * t6275 - 0.53062222222222222222e-1_f64 * t6296 - 0.44218518518518518518e-1_f64 * t1470 * t8212 - 0.53062222222222222222e-1_f64 * t1470 * t8216 + 0.53062222222222222222e-1_f64 * t1470 * t8220 - 0.26531111111111111111e-1_f64 * t1470 * t8224 - 0.39796666666666666666e-1_f64 * t140 * t479 * t8227;
    (t8220, t8224, t8227, t8231)
}
