//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 707/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk707(t1312: f64, t2055: f64, t2322: f64, t5523: f64, t670: f64, t7357: f64, t7359: f64, t7373: f64, t2106: f64, t531: f64, t7238: f64, t2097: f64, t212: f64) -> (f64, f64, f64, f64) {
    let t7484 = 2.0_f64 * t1312 * t7373 + 2.0_f64 * t2055 * t2322 + 2.0_f64 * t2055 * t5523 + 2.0_f64 * t670 * t7359 + t7357;
    let t7488 = t531 * t2106;
    let t7489 = t7488 * t7238;
    let t7492 = t212 * t2097;
    (t7484, t7488, t7489, t7492)
}
