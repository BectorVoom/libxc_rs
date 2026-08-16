//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1037/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1037(t1231: f64, t1216: f64, t1224: f64, t1229: f64, t1230: f64, t1253: f64, t1254: f64, t1262: f64, t1314: f64, t155: f64, t174: f64, t18456: f64, t18471: f64, t18474: f64, t18486: f64, t18488: f64, t18491: f64, t18494: f64, t18500: f64, t18502: f64, t18504: f64, t18506: f64, t18512: f64, t18518: f64, t18527: f64, t18562: f64, t395: f64, t435: f64, t440: f64, t441: f64, t449: f64, t457: f64, t4636: f64, t4674: f64, t4678: f64, t4682: f64, t4734: f64, t4735: f64, t4737: f64, t4738: f64, t7236: f64, t7271: f64, t837: f64) -> (f64, f64) {
    let t18779 = t1231 * t1231;
    let t18801 = -t18456 + t18471 + t18474 - 0.67471169937307261776e-1_f64 * t174 * t837 * t449 * t457 - 0.68493333333333333332e-1_f64 * t174 * t1224 * t4674 - 0.14172186339420759129e3_f64 * t174 * t155 * t4678 * t4682 - 0.38024868119570572865e2_f64 * t174 * t155 * t4734 * t4738 - t18512 - t18518 - t18527 + 0.41096e0_f64 * t395 * t1229 * t440 * t1254 - 0.11579285944033451271e4_f64 * t4636 * t1262 * t1253 - 8.0_f64 * t1230 * t4674 * t440 - 24.0_f64 * t4636 * t18779 * t441 + 1.0_f64 * t435 * (-0.39219166666666666667e1_f64 * t18486 + 0.376504e2_f64 * t18488 - 0.13944592592592592593e2_f64 * t18491 + 0.12201518518518518519e2_f64 * t18494 + 0.5356037037037037037e1_f64 * t7271 + 0.14025833333333333333e0_f64 * t18500 - 0.22441333333333333332e1_f64 * t18502 + 0.24934814814814814815e1_f64 * t18504 + 0.21817962962962962963e1_f64 * t18506 + 0.16979925925925925926e1_f64 * t7236) * t441 + t18562 + 0.61523382126046769581e4_f64 * t4735 * t1216 * t4737 * t1314;
    (t18779, t18801)
}
