//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 963/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk963(t1032: f64, t3736: f64, t3140: f64, t8931: f64, t1276: f64, t2148: f64, t7642: f64, t7657: f64) -> (f64, f64, f64, f64, f64) {
    let t33449 = t1032 * t3736;
    let t33454 = t8931 * t3140;
    let t33455 = t33454 * t1276;
    let t33456 = t2148 * t33455;
    let t33461 = t7642 * t7657;
    (t33449, t33454, t33455, t33456, t33461)
}
