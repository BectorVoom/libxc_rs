//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1236/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1236(t22563: f64, t22575: f64, t22590: f64, t22605: f64, t1005: f64, t6125: f64, t384: f64, t386: f64, t5679: f64, t991: f64, t1901: f64, t3670: f64) -> (f64, f64, f64, f64) {
    let t22607 = t22563 + t22575 + t22590 + t22605;
    let t22613 = t1005 * t6125;
    let t22617 = t384 * t386 * t5679 * t991;
    let t22619 = t3670 * t1901;
    (t22607, t22613, t22617, t22619)
}
