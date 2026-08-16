//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 688/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk688(t5: f64, t1923: f64, t2048: f64, t6954: f64, t6960: f64, t6963: f64, t7343: f64, t7351: f64, t7352: f64, t117: f64, t116: f64, t2051: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7356 = piecewise3(t8, 0.0_f64, t6954 * t2048 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t7343 * t6960 - 2.0_f64 / 3.0_f64 * t6963 * t2048 - t7351 + t1923 * t7352 / 3.0_f64);
    let t7357 = t7356 * t117;
    let t7359 = t2051 * t116;
    (t7356, t7357, t7359)
}
