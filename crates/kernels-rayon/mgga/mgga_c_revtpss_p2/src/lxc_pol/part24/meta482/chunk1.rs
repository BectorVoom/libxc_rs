//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1473/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473(t1235: f64, t371: f64, t6645: f64, t676: f64, t17307: f64, t1803: f64, t11262: f64, t3711: f64, t6618: f64, t3609: f64, t69692: f64, t17416: f64, t5381: f64) -> (f64, f64, f64, f64, f64) {
    let t70263 = t1235 * t371 * t676 * t6645;
    let t70267 = t17307 * t1803;
    let t70278 = t3711 * t11262 * t6618;
    let t70319 = t69692 * t3609;
    let t70405 = t5381 * t17416;
    (t70263, t70267, t70278, t70319, t70405)
}
