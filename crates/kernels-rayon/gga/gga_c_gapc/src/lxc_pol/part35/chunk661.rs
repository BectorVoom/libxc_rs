//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 661/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk661(t144: f64, t195: f64, t102: f64, t1946: f64, t675: f64, t681: f64, t1: f64, t567: f64, t350: f64, t505: f64, t3712: f64, t5054: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5581 = t195 * t144;
    let t5589 = t1946 * t102;
    let t5623 = t675 * t681;
    let t5624 = t567 * t1;
    let t5625 = t5624 * t350;
    let t5626 = t5623 * t5625;
    let t5631 = t505 * t1;
    let t5632 = t5631 * t350;
    let t5633 = t3712 * t5632;
    let t5658 = 1.0_f64 / t8 / t5054;
    (t5581, t5589, t5625, t5626, t5633, t5658)
}
