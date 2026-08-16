//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 843/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk843(t1087: f64, t3406: f64, t829: f64, t9786: f64, t3434: f64, t954: f64, t7204: f64, t9645: f64, t2706: f64, t3103: f64, t3397: f64, t7073: f64, t8673: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9787 = t1087 * t3406;
    let t9788 = t829 * t9787;
    let t9789 = t9786 * t9788;
    let t9791 = t3434 * t954;
    let t9793 = t7204 * t9645;
    let t9795 = t2706 * t3103;
    let t9796 = t9795 * t3397;
    let t9798 = t7073 * t8673;
    (t9787, t9789, t9791, t9793, t9796, t9798)
}
