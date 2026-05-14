//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 831/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk831<F: Float>(t1231: F, t1216: F, t1224: F, t1229: F, t1230: F, t1253: F, t1254: F, t1262: F, t1314: F, t155: F, t174: F, t18456: F, t18471: F, t18474: F, t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t18512: F, t18518: F, t18527: F, t18562: F, t395: F, t435: F, t440: F, t441: F, t449: F, t457: F, t4636: F, t4674: F, t4678: F, t4682: F, t4734: F, t4735: F, t4737: F, t4738: F, t7236: F, t7271: F, t837: F) -> (F, F) {
    let t18779 = t1231 * t1231;
    let t18801 = -t18456 + t18471 + t18474 - 0.67471169937307261776e-1 * t174 * t837 * t449 * t457 - 0.68493333333333333332e-1 * t174 * t1224 * t4674 - 0.14172186339420759129e3 * t174 * t155 * t4678 * t4682 - 0.38024868119570572865e2 * t174 * t155 * t4734 * t4738 - t18512 - t18518 - t18527 + 0.41096e0 * t395 * t1229 * t440 * t1254 - 0.11579285944033451271e4 * t4636 * t1262 * t1253 - 8.0 * t1230 * t4674 * t440 - 24.0 * t4636 * t18779 * t441 + 1.0 * t435 * (-0.39219166666666666667e1 * t18486 + 0.376504e2 * t18488 - 0.13944592592592592593e2 * t18491 + 0.12201518518518518519e2 * t18494 + 0.5356037037037037037e1 * t7271 + 0.14025833333333333333e0 * t18500 - 0.22441333333333333332e1 * t18502 + 0.24934814814814814815e1 * t18504 + 0.21817962962962962963e1 * t18506 + 0.16979925925925925926e1 * t7236) * t441 + t18562 + 0.61523382126046769581e4 * t4735 * t1216 * t4737 * t1314;
    (t18779, t18801)
}
