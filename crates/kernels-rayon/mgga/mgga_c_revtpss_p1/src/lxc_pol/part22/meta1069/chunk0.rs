//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3822/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3822(t162: f64, t73444: f64, t73470: f64, t189: f64, t512: f64, t21931: f64, t749: f64, t22212: f64, t2516: f64, t1868: f64, t4144: f64, t72: f64, t757: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73472 = (t73444 + t73470) * t162;
    let t73474 = t512 * t73472 * t189;
    let t73476 = t512 * t21931 * t749;
    let t73477 = 2.0_f64 * t73476;
    let t73481 = t22212 * t2516;
    let t73482 = 0.5848223622634646207e0_f64 * t73481;
    let t73488 = t1868 * t4144;
    let t73493 = t21931 * t72 * t757;
    (t73472, t73474, t73477, t73482, t73488, t73493)
}
