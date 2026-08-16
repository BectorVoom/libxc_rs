//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1234/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1234(t13278: f64, t5619: f64, t1512: f64, t59281: f64, t67441: f64, t816: f64, t20978: f64, t9638: f64, t20938: f64, t838: f64, t20953: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67852 = t13278 * t5619;
    let t67854 = t59281 * t1512;
    let t67872 = t67441 * t816;
    let t67880 = t9638 * t20978;
    let t67882 = t20938 * t838;
    let t67884 = t2639 * t20953;
    (t67852, t67854, t67872, t67880, t67882, t67884)
}
