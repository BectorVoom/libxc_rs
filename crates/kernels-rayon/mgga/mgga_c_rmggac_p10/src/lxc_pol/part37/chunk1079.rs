//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1079/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1079(t75519: f64, t69828: f64, t73375: f64, t75458: f64, t75461: f64, t75490: f64, t75495: f64, t75500: f64, t77630: f64, t77631: f64, t77633: f64, t77634: f64, t77635: f64, t77636: f64, t77641: f64, t77642: f64, t77643: f64) -> f64 {
    let t80256 = 0.24527028530061914062e-5_f64 * t75519;
    let t80257 = 0.10511583655740820312e-5_f64 * t75458 - 0.10511583655740820312e-5_f64 * t75461 - t77630 - t77631 + t77633 + t77634 - t77635 + t77636 - t73375 - t75490 - t75495 + t75500 + t77641 - t77642 + t77643 - t80256 - t69828;
    t80257
}
