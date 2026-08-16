//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3191/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191(t15438: f64, t15548: f64, t15569: f64, t15608: f64, t15689: f64, t4889: f64, t1174: f64, t135: f64, t18996: f64, t11665: f64, t15650: f64, t18969: f64, t3440: f64, t45197: f64, t5005: f64, t52704: f64, t52897: f64, t53064: f64, t53067: f64, t53079: f64, t53093: f64, t53096: f64, t53099: f64, t53102: f64, t53176: f64, t63315: f64) -> f64 {
    let t66255 = t15438 * t15548;
    let t66268 = t15569 * t15608;
    let t66273 = t4889 * t15689;
    let t66276 = t1174 * t135 * t18996;
    let t66282 = -t66255 / 1152.0_f64 - t53064 / 1728.0_f64 + 5.0_f64 / 10368.0_f64 * t53067 + t45197 * t52897 * t52704 * t53176 / 128.0_f64 + t1174 * t3440 * t63315 / 216.0_f64 - t11665 * t18969 / 2304.0_f64 + t66268 / 324.0_f64 + t53079 / 5184.0_f64 + t53093 / 384.0_f64 + t53096 / 162.0_f64 + t66273 / 81.0_f64 - t66276 / 432.0_f64 + t53099 / 5184.0_f64 - t53102 / 576.0_f64 - t5005 * t15650 / 576.0_f64;
    t66282
}
