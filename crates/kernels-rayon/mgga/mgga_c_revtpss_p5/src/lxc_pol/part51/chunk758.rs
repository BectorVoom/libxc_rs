//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 758/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk758(t532: f64, t8598: f64, t4147: f64, t2014: f64, t118: f64, t1932: f64, t2007: f64, t508: f64, t569: f64, t8447: f64, t8449: f64, t8456: f64, t8458: f64, t8463: f64, t8557: f64, t8565: f64, t8569: f64, t8597: f64) -> (f64, f64, f64) {
    let t8599 = t532 * t8598;
    let t8600 = t8599 * t4147;
    let t8601 = t2014 * t8600;
    let t8602 = -t118 * t8557 - 2.0_f64 * t1932 * t2007 - t508 * t8447 + t569 * t8565 - 4.0_f64 * t8449 - t8456 - 4.0_f64 * t8458 - t8463 + 2.0_f64 * t8569 + t8597 - t8601;
    (t8599, t8600, t8602)
}
