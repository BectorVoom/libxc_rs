//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 816/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk816(t467: f64, t560: f64, t1427: f64, t8034: f64, t5439: f64, t8040: f64, t104: f64, t2407: f64, t1614: f64, t2176: f64, t1410: f64, t157: f64, t2152: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9098 = t560 * t467;
    let t9108 = t8034 * t1427;
    let t9114 = t8040 * t5439;
    let t9121 = t104 * t2407;
    let t9129 = t2176 * t1614;
    let t9136 = t2152 * t633 * t1410 * t157;
    (t9098, t9108, t9114, t9121, t9129, t9136)
}
