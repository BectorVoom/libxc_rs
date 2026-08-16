//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2734/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2734(t10777: f64, t10779: f64, t1548: f64, t2646: f64, t10868: f64, t820: f64, t844: f64, t14896: f64, t14701: f64, t40731: f64, t14468: f64, t221: f64, t2674: f64, t2675: f64) -> (f64, f64, f64, f64) {
    let t50292 = t10777 * t10779 * t1548 * t2646;
    let t50295 = t820 * t10868 * t844;
    let t50296 = t50295 * t14896;
    let t50298 = t40731 * t14701;
    let t50299 = 0.16262400898971305032e-2_f64 * t50298;
    let t50303 = t2674 * t2675 * t221 * t14468;
    (t50292, t50296, t50299, t50303)
}
