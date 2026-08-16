//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 674/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk674(t4980: f64, t36: f64, t409: f64, t89: f64, t1385: f64, t732: f64, t1380: f64, t453: f64, t4811: f64, t234: f64, t1409: f64, t1497: f64, t454: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4981 = 24.0_f64 * t4980;
    let t4982 = t36 * t409;
    let t4983 = t4982 * t89;
    let t4987 = t732 * t1385;
    let t4990 = t1380 * t4811 * t453;
    let t4991 = t234 * t4990;
    let t4992 = 0.35089341735807877242e1_f64 * t4991;
    let t4994 = t1497 * t1409 * t454;
    (t4981, t4982, t4983, t4987, t4992, t4994)
}
