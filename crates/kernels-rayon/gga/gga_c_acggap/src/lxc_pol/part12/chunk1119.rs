//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1119/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1119(t1322: f64, t721: f64, t7859: f64, t2041: f64, t4632: f64, t1426: f64, t429: f64, t598: f64, t8539: f64, t35500: f64, t7380: f64, t34050: f64) -> (f64, f64, f64, f64, f64) {
    let t35885 = t7859 * t1322 * t721;
    let t35887 = t2041 * t4632;
    let t35907 = t598 * t1426 * t429 * t8539;
    let t35909 = t7380 * t35500;
    let t35911 = t7380 * t34050;
    (t35885, t35887, t35907, t35909, t35911)
}
