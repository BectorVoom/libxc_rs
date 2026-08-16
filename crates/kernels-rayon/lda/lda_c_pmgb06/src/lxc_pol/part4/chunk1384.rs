//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1384/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1384(t11810: f64, t11813: f64, t15509: f64, t15510: f64, t15511: f64, t15516: f64, t15518: f64, t15520: f64, t15522: f64, t15524: f64, t15526: f64, t15527: f64, t15699: f64, t9338: f64, t9340: f64) -> f64 {
    let t18173 = 0.003030876351851852_f64 * t11810 - 0.027012345679012346_f64 * t11813 - t15509 + t15510 + t15511 - t15516 + t15518 - t15520 - t15522 + t15524 - t15526 + t15527 - t15699 + 0.033245444444444446_f64 * t9338 + 0.19947266666666666_f64 * t9340;
    t18173
}
