//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 612/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk612(t3896: f64, t464: f64, t852: f64, t880: f64, t441: f64, t851: f64, t323: f64, t1222: f64, t857: f64, t872: f64, t1221: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3897 = t3896 * t464;
    let t3900 = 0.19756347548806534796e1_f64 * t852 * t880;
    let t3901 = t851 * t441;
    let t3902 = t3901 * t323;
    let t3904 = t857 * t1222;
    let t3906 = t852 * t872;
    let t3908 = t322 * t1221;
    (t3897, t3900, t3901, t3902, t3904, t3906, t3908)
}
