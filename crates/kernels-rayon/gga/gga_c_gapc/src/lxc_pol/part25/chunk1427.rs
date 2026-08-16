//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1427/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1427(t12654: f64, t699: f64, t1617: f64, t3909: f64, t4915: f64, t1615: f64, t3903: f64, t12585: f64, t575: f64, t687: f64, t12587: f64, t2011: f64) -> (f64, f64, f64, f64, f64) {
    let t38695 = t699 * t12654;
    let t38699 = 6.0_f64 * t4915 * t3909 * t1617;
    let t38700 = t3903 * t1615;
    let t38702 = 2.0_f64 * t38700 * t1617;
    let t38703 = t12585 * t575;
    let t38705 = 2.0_f64 * t38703 * t687;
    let t38706 = t12587 * t2011;
    (t38695, t38699, t38702, t38705, t38706)
}
