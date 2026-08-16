//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 871/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk871(t435: f64, t965: f64, t3216: f64, t3357: f64, t3652: f64, t3657: f64, t3806: f64, t3207: f64, t363: f64, t1080: f64, t987: f64, t656: f64, t668: f64, t682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12610 = t965 * t435;
    let t12615 = t3216 * t3357;
    let t12621 = t3216 * t3652;
    let t12623 = t3216 * t3657;
    let t12626 = 0.24009450146119052704e-1_f64 * t3216 * t3806;
    let t12641 = t3207 * t363;
    let t12646 = t987 * t1080;
    let t12661 = 0.43374325201206959368e-1_f64 * t656 * t668 * t682;
    (t12610, t12615, t12621, t12623, t12626, t12641, t12646, t12661)
}
