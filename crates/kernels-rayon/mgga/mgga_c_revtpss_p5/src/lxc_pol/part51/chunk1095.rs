//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1095/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1095(t108120: f64, t1937: f64, t28030: f64, t6993: f64, t4147: f64, t5591: f64, t2014: f64, t32119: f64, t28043: f64, t6985: f64, t28182: f64, t8568: f64) -> (f64, f64, f64, f64, f64) {
    let t125442 = t108120 * t1937;
    let t125444 = t28030 * t6993;
    let t125453 = t4147 * t5591;
    let t125456 = 3.0_f64 * t2014 * t32119 * t125453;
    let t125459 = t6985 * t28043;
    let t125467 = t8568 * t28182;
    (t125442, t125444, t125456, t125459, t125467)
}
