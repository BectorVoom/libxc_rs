//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 782/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk782(t23472: f64, t23473: f64, t40: f64, t984: f64, t1933: f64, t225: f64, t343: f64, t364: f64, t6721: f64, t6739: f64, t6741: f64, t1937: f64, t23447: f64, t23449: f64, t23454: f64, t23457: f64, t23460: f64, t23463: f64, t23465: f64, t23469: f64, t350: f64, t378: f64, t6747: f64) -> (f64, f64) {
    let t23474 = t23472 * t23473;
    let t23476 = t40 * t984;
    let t23477 = t1933 * t23476;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    let t23483 = t23482 * t6741;
    let t23486 = -t23447 - 0.16149102437656156342e-2_f64 * t23449 + 0.72670960969452703541e-2_f64 * t23454 * t1937 - 0.16149102437656156342e-2_f64 * t23457 * t1937 + 11.0_f64 / 108.0_f64 * t23460 * t350 - t23463 / 54.0_f64 + t23465 * t378 / 1536.0_f64 - t23469 + 0.20186378047070195428e-3_f64 * t23474 - 0.20186378047070195428e-3_f64 * t23480 - 0.16149102437656156342e-2_f64 * t23483 * t6747;
    (t23476, t23486)
}
