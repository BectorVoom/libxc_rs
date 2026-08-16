//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 776/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk776(t4561: f64, t6713: f64, t4570: f64, t6724: f64, t2030: f64, t4656: f64, t4652: f64, t4579: f64, t591: f64, t40: f64, t1: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13064 = t6713 * t4561;
    let t13076 = t6724 * t4570;
    let t13092 = t2030 * t4656;
    let t13094 = t2030 * t4652;
    let t13110 = t4579 * t591;
    let t13111 = t40 * t13110;
    let t13113 = t4579 * t1;
    let t13114 = t13113 * t598;
    (t13064, t13076, t13092, t13094, t13110, t13111, t13113, t13114)
}
