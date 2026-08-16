//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 939/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk939(t1236: f64, t14575: f64, t3088: f64, t1248: f64, t980: f64, t3930: f64, t872: f64, t3858: f64, t880: f64, t13326: f64, t188: f64, t3901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14577 = t3088 * t1236 * t14575;
    let t14579 = t980 * t1248;
    let t14591 = t3930 * t872;
    let t14593 = t3858 * t880;
    let t14606 = 0.65854491829355115987e0_f64 * t13326 * t188;
    let t14616 = t3901 * t880;
    (t14577, t14579, t14591, t14593, t14606, t14616)
}
