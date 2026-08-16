//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3195/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195(t27524: f64, t607: f64, t1215: f64, t6224: f64, t1227: f64, t13969: f64, t18954: f64, t11709: f64, t15617: f64, t15702: f64, t15708: f64, t15709: f64, t15740: f64, t15750: f64, t18236: f64, t18397: f64, t18948: f64, t19002: f64, t3247: f64, t3508: f64, t3577: f64, t3578: f64, t45112: f64, t45119: f64, t45134: f64, t45162: f64, t5005: f64, t53220: f64, t53246: f64, t66360: f64, t66363: f64, t66372: f64, t66374: f64, t66378: f64) -> (f64, f64) {
    let t66380 = t27524 * t607;
    let t66388 = t6224 * t1215;
    let t66398 = t1227 * t13969 * t18954;
    let t66400 = -t53220 / 384.0_f64 - t5005 * t15617 / 384.0_f64 - t66360 / 1728.0_f64 - t66363 / 3456.0_f64 - t3577 * t3578 * t18236 * t15708 / 1152.0_f64 - t45112 + t11709 * t18948 / 384.0_f64 + t66372 * t66374 * t15709 / 576.0_f64 - t66378 * t66374 * t3508 * t3247 * t66380 / 288.0_f64 + t53246 / 648.0_f64 + 5.0_f64 / 3456.0_f64 * t15740 * t15750 - t45119 * t3578 * t66388 * t15702 / 2304.0_f64 - t45162 * t19002 / 576.0_f64 + t45134 * t18397 / 1152.0_f64 - 5.0_f64 / 3888.0_f64 * t66398;
    (t66380, t66400)
}
