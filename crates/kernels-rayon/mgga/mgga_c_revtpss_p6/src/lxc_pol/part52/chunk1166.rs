//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1166/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1166(t121019: f64, t32284: f64, t5700: f64, t121018: f64, t1399: f64, t33962: f64, t34230: f64, t4075: f64, t121116: f64, t33930: f64, t1389: f64, t32282: f64) -> (f64, f64, f64, f64, f64) {
    let t125599 = t32284 * t121019 * t5700;
    let t125603 = t121018 * t121019 * t33962 * t1399;
    let t125609 = t34230 * t4075;
    let t125617 = t121116 * t33930;
    let t125625 = t32282 * t1389;
    (t125599, t125603, t125609, t125617, t125625)
}
