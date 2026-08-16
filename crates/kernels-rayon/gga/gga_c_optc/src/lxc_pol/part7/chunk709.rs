//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 709/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk709(t6756: f64, t743: f64, t1911: f64, t1916: f64, t188: f64, t1972: f64, t712: f64, t171: f64, t1974: f64, t2045: f64, t592: f64, t2042: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6757 = t743 * t6756;
    let t6760 = t1916 * t1911;
    let t6761 = t188 * t6760;
    let t6763 = t1972 * t712;
    let t6766 = 1.0_f64 / t1974 / t171;
    let t6770 = t2045 * t592;
    let t6771 = 36.0_f64 * t6770;
    let t6772 = t2042 * t559;
    (t6757, t6760, t6761, t6763, t6766, t6771, t6772)
}
