//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1926/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1926(t3287: f64, t4756: f64, t1102: f64, t3279: f64, t4764: f64, t4772: f64, t699: f64) -> (f64, f64, f64) {
    let t14813 = t3287 * t4756;
    let t14814 = t14813 * t1102;
    let t14816 = t4764 * t3279;
    let t14818 = t699 * t4772;
    (t14814, t14816, t14818)
}
