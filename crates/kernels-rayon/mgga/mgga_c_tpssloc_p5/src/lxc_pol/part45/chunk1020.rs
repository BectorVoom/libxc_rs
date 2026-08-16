//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1020/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1020(t225: f64, t31573: f64, t1985: f64, t22666: f64, t31607: f64, t31590: f64, t6883: f64, t114145: f64, t114150: f64, t114155: f64, t114159: f64, t114163: f64, t114168: f64, t114171: f64, t114175: f64, t115469: f64, t1386: f64, t2016: f64, t22630: f64, t22905: f64, t31642: f64, t31653: f64, t3882: f64, t3889: f64, t539: f64, t568: f64, t7194: f64, t84655: f64) -> f64 {
    let t115519 = t31573 * t225;
    let t115523 = t1985 * t22666 * t31607;
    let t115530 = t6883 * t31590;
    let t115532 = -6.0_f64 * t7194 * t22630 - t7194 * t22905 + t539 * t115469 * t568 + t114145 - 2.0_f64 * t115519 * t1386 - t114150 - 0.16449340668482264365e-1_f64 * t115523 + t114155 + t114159 - t114163 - t84655 * t2016 - t114168 - t114171 - 2.0_f64 * t3882 * t31642 + t114175 + 2.0_f64 * t31653 * t3889 - 0.38381794893125283518e-1_f64 * t115530;
    t115532
}
