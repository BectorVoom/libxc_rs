//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2752/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752(t2608: f64, t512: f64, t6800: f64, t177: f64, t21931: f64, t762: f64, t1320: f64, t22193: f64, t22461: f64, t4147: f64, t749: f64, t22212: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73350 = t512 * t6800 * t2608;
    let t73352 = t21931 * t177 * t762;
    let t73374 = t1320 * t22193;
    let t73407 = t22461 * t4147;
    let t73476 = t512 * t21931 * t749;
    let t73481 = t22212 * t2516;
    (t73350, t73352, t73374, t73407, t73476, t73481)
}
