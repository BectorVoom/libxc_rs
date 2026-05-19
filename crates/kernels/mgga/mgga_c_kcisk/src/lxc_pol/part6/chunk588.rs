//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 588/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk588<F: Float>(t1471: F, t4282: F, t7706: F, t1472: F, t7710: F, t416: F, t8159: F, t140: F, t1470: F, t2221: F, t2225: F, t2242: F, t4253: F, t4264: F, t460: F, t476: F, t479: F, t6275: F, t6296: F, t7865: F, t7869: F, t7873: F, t7878: F, t7898: F, t8192: F, t8212: F, t8216: F) -> (F, F, F, F) {
    let t8220 = t1471 * t4282 * t7706;
    let t8224 = t1471 * t1472 * t7710;
    let t8227 = t416 * t8159;
    let t8231 = F::new(0.619125e-2) * t8192 * t460 + F::new(0.1857375e-1) * t2242 * t2221 - F::new(0.123825e-1) * t2242 * t2225 + F::new(0.46434375e-2) * t476 * t7865 - F::new(0.1857375e-1) * t4253 * t7869 + F::new(0.9286875e-2) * t476 * t7873 + F::new(0.123825e-1) * t476 * t7878 - F::new(0.619125e-2) * t476 * t7898 + t4264 - F::cast_from(0.35374814814814814814e-1_f64) * t6275 - F::cast_from(0.53062222222222222222e-1_f64) * t6296 - F::cast_from(0.44218518518518518518e-1_f64) * t1470 * t8212 - F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t8216 + F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t8220 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t8224 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t8227;
    (t8220, t8224, t8227, t8231)
}
