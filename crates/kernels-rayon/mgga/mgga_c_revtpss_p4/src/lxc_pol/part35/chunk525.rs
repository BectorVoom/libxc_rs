//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 525/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk525(t550: f64, t72: f64, t245: f64, t125: f64, t1882: f64, t1873: f64, t3957: f64, t1892: f64, t213: f64, t1357: f64, t1904: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5674 = t125 * t1882;
    let t5681 = t3957 * t1873;
    let t5715 = t213 * t1892;
    let t5718 = t1357 * t1904;
    let t5719 = t689 * t5718;
    (t5673, t5674, t5681, t5715, t5718, t5719)
}
